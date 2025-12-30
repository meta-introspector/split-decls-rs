// Generated macro for impl_105 (impl)
macro_rules! Depcrate_constraints_graphimpl_105 {
() => {
// Module: crate::constraints::graph
// Provides: {"impl_105"}
// Dependencies: {}
impl ConstraintGraphDirection for Reverse { fn start_region (_sup : RegionVid , sub : RegionVid) -> RegionVid { sub } fn end_region (sup : RegionVid , _sub : RegionVid) -> RegionVid { sup } fn is_normal () -> bool { false } }
};
}
