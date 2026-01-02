// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_ssa/src/mir/analyze.rs
// Error: expected square brackets
// Problematic line: line 17

use super::FunctionCx;
use crate::traits::*;

pub(crate) fn non_ssa_locals<'a, 'tcx, Bx: BuilderMethods<'a, 'tcx>>(
    fx: &FunctionCx<'a, 'tcx, Bx>,
    traversal_order: &[mir::BasicBlock],
) -> DenseBitSet<mir::Local> {
