// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_cranelift/src/abi/returning.rs
// Error: expected square brackets
// Problematic line: line 8


use crate::prelude::*;

/// Return a place where the return value of the current function can be written to. If necessary
/// this adds an extra parameter pointing to where the return value needs to be stored.
pub(super) fn codegen_return_param<'tcx>(
    fx: &mut FunctionCx<'_, '_, 'tcx>,
