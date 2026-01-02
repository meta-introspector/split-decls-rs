// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_hir_analysis/src/coherence/unsafety.rs
// Error: expected square brackets
// Problematic line: line 13

use rustc_span::ErrorGuaranteed;
use rustc_span::def_id::LocalDefId;

pub(super) fn check_item(
    tcx: TyCtxt<'_>,
    def_id: LocalDefId,
    trait_header: ImplTraitHeader<'_>,
