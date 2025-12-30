// Generated macro for impl_273 (impl)
macro_rules! Depcrate_diagnostics_bound_region_errorsimpl_273 {
() => {
// Module: crate::diagnostics::bound_region_errors
// Provides: {"impl_273"}
// Dependencies: {}
impl < 'tcx > ToUniverseInfo < 'tcx > for CanonicalTypeOpProvePredicateGoal < 'tcx > { fn to_universe_info (self , base_universe : ty :: UniverseIndex) -> UniverseInfo < 'tcx > { UniverseInfo :: TypeOp (Rc :: new (PredicateQuery { canonical_query : self , base_universe })) } }
};
}
