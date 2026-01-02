// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_cranelift/src/intrinsics/llvm_x86.rs
// Error: expected square brackets
// Problematic line: line 10

use crate::intrinsics::*;
use crate::prelude::*;

pub(super) fn codegen_x86_llvm_intrinsic_call<'tcx>(
    fx: &mut FunctionCx<'_, '_, 'tcx>,
    intrinsic: &str,
    args: &[Spanned<mir::Operand<'tcx>>],
