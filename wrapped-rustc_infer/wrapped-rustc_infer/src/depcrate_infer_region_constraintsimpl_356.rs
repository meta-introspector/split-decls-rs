// Generated macro for impl_356 (impl)
macro_rules! Depcrate_infer_region_constraintsimpl_356 {
() => {
// Module: crate::infer::region_constraints
// Provides: {"impl_356"}
// Dependencies: {}
impl < 'tcx > RegionConstraintData < 'tcx > { # [doc = " Returns `true` if this region constraint data contains no constraints, and `false`"] # [doc = " otherwise."] pub fn is_empty (& self) -> bool { let RegionConstraintData { constraints , verifys } = self ; constraints . is_empty () && verifys . is_empty () } }
};
}
