// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_cranelift/src/global_asm.rs
// Error: expected square brackets
// Problematic line: line 12

use rustc_ast::{InlineAsmOptions, InlineAsmTemplatePiece};
use rustc_codegen_ssa::traits::{AsmCodegenMethods, GlobalAsmOperandRef};
use rustc_middle::ty::TyCtxt;
use rustc_middle::ty::layout::{
    FnAbiError, FnAbiOfHelpers, FnAbiRequest, HasTyCtxt, HasTypingEnv, LayoutError, LayoutOfHelpers,
};
use rustc_session::config::{OutputFilenames, OutputType};
