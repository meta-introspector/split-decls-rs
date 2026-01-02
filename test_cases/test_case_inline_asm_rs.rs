// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_cranelift/src/inline_asm.rs
// Error: expected square brackets
// Problematic line: line 14


use crate::prelude::*;

pub(crate) enum CInlineAsmOperand<'tcx> {
    In {
        reg: InlineAsmRegOrRegClass,
        value: Value,
