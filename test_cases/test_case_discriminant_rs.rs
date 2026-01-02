// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_cranelift/src/discriminant.rs
// Error: expected square brackets
// Problematic line: line 11


use crate::prelude::*;

pub(crate) fn codegen_set_discriminant<'tcx>(
    fx: &mut FunctionCx<'_, '_, 'tcx>,
    place: CPlace<'tcx>,
    variant_index: VariantIdx,
