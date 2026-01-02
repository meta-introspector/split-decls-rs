// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_cranelift/src/cast.rs
// Error: expected square brackets
// Problematic line: line 6

use crate::codegen_f16_f128;
use crate::prelude::*;

pub(crate) fn clif_intcast(
    fx: &mut FunctionCx<'_, '_, '_>,
    val: Value,
    to: Type,
