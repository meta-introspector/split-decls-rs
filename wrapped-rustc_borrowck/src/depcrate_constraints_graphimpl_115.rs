// Generated macro for impl_115 (impl)
macro_rules! Depcrate_constraints_graphimpl_115 {
() => {
// Module: crate::constraints::graph
// Provides: {"impl_115"}
// Dependencies: {}
impl < 'a , 'tcx , D : ConstraintGraphDirection > graph :: DirectedGraph for RegionGraph < 'a , 'tcx , D > { type Node = RegionVid ; fn num_nodes (& self) -> usize { self . constraint_graph . first_constraints . len () } }
};
}
