// Generated macro for impl_276 (impl)
macro_rules! Depcrate_diagnostics_bound_region_errorsimpl_276 {
() => {
// Module: crate::diagnostics::bound_region_errors
// Provides: {"impl_276"}
// Dependencies: {}
impl < 'tcx > ToUniverseInfo < 'tcx > for CanonicalTypeOpAscribeUserTypeGoal < 'tcx > { fn to_universe_info (self , base_universe : ty :: UniverseIndex) -> UniverseInfo < 'tcx > { UniverseInfo :: TypeOp (Rc :: new (AscribeUserTypeQuery { canonical_query : self , base_universe })) } }
};
}
