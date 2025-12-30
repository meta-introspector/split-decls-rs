// Generated macro for impl_267 (impl)
macro_rules! Depcrate_graph_vec_graphimpl_267 {
() => {
// Module: crate::graph::vec_graph
// Provides: {"impl_267"}
// Dependencies: {}
impl < N : Idx , const BR : bool > NumEdges for VecGraph < N , BR > { fn num_edges (& self) -> usize { match BR { false => self . edge_targets . len () , true => self . edge_targets . len () / 2 , } } }
};
}
