// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_pattern_analysis/src/rustc/print.rs
// Error: expected square brackets
// Problematic line: line 19

use rustc_middle::ty::{self, AdtDef, Ty, TyCtxt};
use rustc_span::sym;

#[derive(Clone, Debug)]
pub(crate) struct FieldPat {
    pub(crate) field: FieldIdx,
    pub(crate) pattern: String,
