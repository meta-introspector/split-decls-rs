// Generated macro for impl_275 (impl)
macro_rules! Depcrate_diagnostics_bound_region_errorsimpl_275 {
() => {
// Module: crate::diagnostics::bound_region_errors
// Provides: {"impl_275"}
// Dependencies: {}
impl < 'tcx , T : Copy + fmt :: Display + TypeFoldable < TyCtxt < 'tcx > > + 'tcx > ToUniverseInfo < 'tcx > for CanonicalTypeOpDeeplyNormalizeGoal < 'tcx , T > { fn to_universe_info (self , base_universe : ty :: UniverseIndex) -> UniverseInfo < 'tcx > { UniverseInfo :: TypeOp (Rc :: new (DeeplyNormalizeQuery { canonical_query : self , base_universe })) } }
};
}
