// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::support::dummy_procedure_module;

use move_binary_format::{
    
    file_format::{Bytecode},
};

use move_bytecode_verifier::CodeUnitVerifier;

#[test]
fn invalid_overflow() {
    
    let module = dummy_procedure_module(vec![Bytecode::LdU64(3323940208748926750),Bytecode::VecUnpack(move_binary_format::file_format::SignatureIndex::new(2), 3315214543476364830),Bytecode::VecUnpack(move_binary_format::file_format::SignatureIndex::new(2), 18394158839224997406), Bytecode::Ret, Bytecode::Ret]);
    let result = CodeUnitVerifier::verify_module(&module);

    assert!(result.is_err());
}


