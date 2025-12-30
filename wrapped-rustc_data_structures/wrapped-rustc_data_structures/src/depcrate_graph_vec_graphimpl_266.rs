// Generated macro for impl_266 (impl)
macro_rules! Depcrate_graph_vec_graphimpl_266 {
() => {
// Module: crate::graph::vec_graph
// Provides: {"impl_266"}
// Dependencies: {}
impl < N : Idx , const BR : bool > DirectedGraph for VecGraph < N , BR > { type Node = N ; fn num_nodes (& self) -> usize { match BR { false => self . node_starts . len () - 1 , true => (self . node_starts . len () - 1) / 2 , } } }
};
}
