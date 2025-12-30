// Generated macro for impl_116 (impl)
macro_rules! Depcrate_constraints_graphimpl_116 {
() => {
// Module: crate::constraints::graph
// Provides: {"impl_116"}
// Dependencies: {}
impl < 'a , 'tcx , D : ConstraintGraphDirection > graph :: Successors for RegionGraph < 'a , 'tcx , D > { fn successors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > { self . outgoing_regions (node) } }
};
}
