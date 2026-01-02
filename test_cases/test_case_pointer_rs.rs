// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_cranelift/src/pointer.rs
// Error: expected square brackets
// Problematic line: line 9


use crate::prelude::*;

/// A pointer pointing either to a certain address, a certain stack slot or nothing.
#[derive(Copy, Clone, Debug)]
pub(crate) struct Pointer {
    base: PointerBase,
