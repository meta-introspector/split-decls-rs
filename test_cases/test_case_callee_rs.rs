// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_llvm/src/callee.rs
// Error: expected square brackets
// Problematic line: line 16

use crate::llvm;
use crate::value::Value;

/// Codegens a reference to a fn/method item, monomorphizing and
/// inlining as it goes.
pub(crate) fn get_fn<'ll, 'tcx>(cx: &CodegenCx<'ll, 'tcx>, instance: Instance<'tcx>) -> &'ll Value {
    let tcx = cx.tcx();
