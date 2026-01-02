// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_llvm/src/llvm/enzyme_ffi.rs
// Error: expected square brackets
// Problematic line: line 9

use super::ffi::{AttributeKind, BasicBlock, Metadata, Module, Type, Value};
use crate::llvm::{Bool, Builder};

#[link(name = "llvm-wrapper", kind = "static")]
unsafe extern "C" {
    // Enzyme
    pub(crate) safe fn LLVMRustHasMetadata(I: &Value, KindID: MetadataKindId) -> bool;
