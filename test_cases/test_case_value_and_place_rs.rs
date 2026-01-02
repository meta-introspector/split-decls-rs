// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_cranelift/src/value_and_place.rs
// Error: expected square brackets
// Problematic line: line 11


use crate::prelude::*;

fn codegen_field<'tcx>(
    fx: &mut FunctionCx<'_, '_, 'tcx>,
    base: Pointer,
    extra: Option<Value>,
