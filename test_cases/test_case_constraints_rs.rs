// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_hir_analysis/src/variance/constraints.rs
// Error: expected square brackets
// Problematic line: line 16

use super::terms::VarianceTerm::*;
use super::terms::*;

pub(crate) struct ConstraintContext<'a, 'tcx> {
    pub terms_cx: TermsContext<'a, 'tcx>,

    // These are pointers to common `ConstantTerm` instances
