// Generated macro for impl_103 (impl)
macro_rules! Depcrate_constraints_graphimpl_103 {
() => {
// Module: crate::constraints::graph
// Provides: {"impl_103"}
// Dependencies: {}
impl ConstraintGraphDirection for Normal { fn start_region (sup : RegionVid , _sub : RegionVid) -> RegionVid { sup } fn end_region (_sup : RegionVid , sub : RegionVid) -> RegionVid { sub } fn is_normal () -> bool { true } }
};
}
