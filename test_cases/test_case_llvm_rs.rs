// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_cranelift/src/intrinsics/llvm.rs
// Error: expected square brackets
// Problematic line: line 6

use crate::intrinsics::*;
use crate::prelude::*;

pub(crate) fn codegen_llvm_intrinsic_call<'tcx>(
    fx: &mut FunctionCx<'_, '_, 'tcx>,
    intrinsic: &str,
    args: &[Spanned<mir::Operand<'tcx>>],
