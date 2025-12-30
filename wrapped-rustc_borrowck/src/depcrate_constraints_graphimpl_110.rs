// Generated macro for impl_110 (impl)
macro_rules! Depcrate_constraints_graphimpl_110 {
() => {
// Module: crate::constraints::graph
// Provides: {"impl_110"}
// Dependencies: {}
impl Iterator for EdgesFromStatic { type Item = RegionVid ; fn next (& mut self) -> Option < Self :: Item > { if self . next_static_idx < self . end_static_idx { let ret = RegionVid :: from_usize (self . next_static_idx) ; self . next_static_idx += 1 ; Some (ret) } else { None } } }
};
}
