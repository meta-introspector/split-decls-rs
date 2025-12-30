// Generated macro for impl_242 (impl)
macro_rules! Depcrate_graph_sccimpl_242 {
() => {
// Module: crate::graph::scc
// Provides: {"impl_242"}
// Dependencies: {}
impl < N : Idx , S : Idx + Ord > DirectedGraph for Sccs < N , S > { type Node = S ; fn num_nodes (& self) -> usize { self . num_sccs () } }
};
}
