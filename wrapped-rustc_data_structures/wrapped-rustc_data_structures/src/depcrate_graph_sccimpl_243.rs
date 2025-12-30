// Generated macro for impl_243 (impl)
macro_rules! Depcrate_graph_sccimpl_243 {
() => {
// Module: crate::graph::scc
// Provides: {"impl_243"}
// Dependencies: {}
impl < N : Idx , S : Idx + Ord > NumEdges for Sccs < N , S > { fn num_edges (& self) -> usize { self . scc_data . all_successors . len () } }
};
}
