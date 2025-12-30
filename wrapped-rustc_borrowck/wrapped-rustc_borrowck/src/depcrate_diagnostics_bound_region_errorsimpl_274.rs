// Generated macro for impl_274 (impl)
macro_rules! Depcrate_diagnostics_bound_region_errorsimpl_274 {
() => {
// Module: crate::diagnostics::bound_region_errors
// Provides: {"impl_274"}
// Dependencies: {}
impl < 'tcx , T : Copy + fmt :: Display + TypeFoldable < TyCtxt < 'tcx > > + 'tcx > ToUniverseInfo < 'tcx > for CanonicalTypeOpNormalizeGoal < 'tcx , T > { fn to_universe_info (self , base_universe : ty :: UniverseIndex) -> UniverseInfo < 'tcx > { UniverseInfo :: TypeOp (Rc :: new (NormalizeQuery { canonical_query : self , base_universe })) } }
};
}
