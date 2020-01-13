// Copyright 2020 Materialize, Inc. All rights reserved.
//
// This file is part of Materialize. Materialize may not be used or
// distributed without the express permission of Materialize, Inc.

//! Tools for manipulating proto in testdrive

use protoc::Protoc;
use serde_protobuf::descriptor::Descriptors;

use interchange::protobuf::read_descriptors_from_file;

use crate::action::{Action, State};
use crate::error::Error;
use crate::parser::BuiltinCommand;

/// Testrive Action to Compile a `.proto` file into [`Descriptors`] for use by interchange
pub struct CompileProto {
    source: String,
    dest: String,
}

pub fn compile_protoc(mut cmd: BuiltinCommand) -> Result<CompileProto, Error> {
    Ok(CompileProto {
        source: cmd.args.string("source")?,
        dest: cmd.args.string("dest")?,
    })
}

impl Action for CompileProto {
    // Don't undo anything
    fn undo(&self, _state: &mut State) -> Result<(), String> {
        Ok(())
    }

    fn redo(&self, _state: &mut State) -> Result<(), String> {
        let _ = generate_descriptors(&self.source, &self.dest)?;
        Ok(())
    }
}

/// Takes a path to a .proto spec and attempts to generate a binary file
/// containing a set of descriptors for the message (and any nested messages)
/// defined in the spec.
pub fn generate_descriptors(proto_path: &str, out: &str) -> Result<Descriptors, Error> {
    let protoc = Protoc::from_env_path();
    let descriptor_set_out_args = protoc::DescriptorSetOutArgs {
        out,
        includes: &[],
        input: &[proto_path],
        include_imports: false,
    };

    protoc
        .write_descriptor_set(descriptor_set_out_args)
        .expect("protoc write descriptor set failed");
    Ok(read_descriptors_from_file(out).map_err(|e| format!("{}", e))?)
}
