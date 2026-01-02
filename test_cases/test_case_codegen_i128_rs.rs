// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_cranelift/src/codegen_i128.rs
// Error: expected square brackets
// Problematic line: line 7


use crate::prelude::*;

pub(crate) fn maybe_codegen<'tcx>(
    fx: &mut FunctionCx<'_, '_, 'tcx>,
    bin_op: BinOp,
    lhs: CValue<'tcx>,
