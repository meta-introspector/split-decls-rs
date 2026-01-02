// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_hir_analysis/src/coherence/inherent_impls.rs
// Error: expected square brackets
// Problematic line: line 22


use crate::errors;

/// On-demand query: yields a map containing all types mapped to their inherent impls.
pub(crate) fn crate_inherent_impls(
    tcx: TyCtxt<'_>,
    (): (),
