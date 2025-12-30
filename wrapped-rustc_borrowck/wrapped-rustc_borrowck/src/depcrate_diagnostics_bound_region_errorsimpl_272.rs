// Generated macro for impl_272 (impl)
macro_rules! Depcrate_diagnostics_bound_region_errorsimpl_272 {
() => {
// Module: crate::diagnostics::bound_region_errors
// Provides: {"impl_272"}
// Dependencies: {}
impl < 'tcx > ToUniverseInfo < 'tcx > for crate :: type_check :: InstantiateOpaqueType < 'tcx > { fn to_universe_info (self , base_universe : ty :: UniverseIndex) -> UniverseInfo < 'tcx > { UniverseInfo :: TypeOp (Rc :: new (crate :: type_check :: InstantiateOpaqueType { base_universe : Some (base_universe) , .. self })) } }
};
}
