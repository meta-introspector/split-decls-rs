// Generated macro for compute_closure_requirements_modulo_opaques (function)
macro_rules! Depcrate_nllcompute_closure_requirements_modulo_opaques {
() => {
// Module: crate::nll
// Provides: {"compute_closure_requirements_modulo_opaques"}
// Dependencies: {}
# [doc = " Computes the closure requirements given the current inference state."] # [doc = ""] # [doc = " This is intended to be used by before [BorrowCheckRootCtxt::handle_opaque_type_uses]"] # [doc = " because applying member constraints may rely on closure requirements."] # [doc = " This is frequently the case of async functions where pretty much everything"] # [doc = " happens inside of the inner async block but the opaque only gets constrained"] # [doc = " in the parent function."] pub (crate) fn compute_closure_requirements_modulo_opaques < 'tcx > (infcx : & BorrowckInferCtxt < 'tcx > , body : & Body < 'tcx > , location_map : Rc < DenseLocationMap > , universal_region_relations : & Frozen < UniversalRegionRelations < 'tcx > > , constraints : & MirTypeckRegionConstraints < 'tcx > ,) -> Option < ClosureRegionRequirements < 'tcx > > { let lowered_constraints = compute_sccs_applying_placeholder_outlives_constraints (constraints . clone () , & universal_region_relations , infcx ,) ; let mut regioncx = RegionInferenceContext :: new (& infcx , lowered_constraints , universal_region_relations . clone () , location_map ,) ; let (closure_region_requirements , _nll_errors) = regioncx . solve (infcx , body , None) ; closure_region_requirements }
};
}
