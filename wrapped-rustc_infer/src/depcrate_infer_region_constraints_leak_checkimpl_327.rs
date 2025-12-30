// Generated macro for impl_327 (impl)
macro_rules! Depcrate_infer_region_constraints_leak_checkimpl_327 {
() => {
// Module: crate::infer::region_constraints::leak_check
// Provides: {"impl_327"}
// Dependencies: {}
impl < 'tcx > SccUniverse < 'tcx > { # [doc = " If `universe` is less than our current universe, then update"] # [doc = " `self.universe` and `self.region`."] fn take_min (& mut self , universe : ty :: UniverseIndex , region : ty :: Region < 'tcx >) { if universe < self . universe || self . region . is_none () { self . universe = universe ; self . region = Some (region) ; } } }
};
}
