// Generated macro for impl_188 (impl)
macro_rules! Depcrate_graph_reversedimpl_188 {
() => {
// Module: crate::graph::reversed
// Provides: {"impl_188"}
// Dependencies: {}
impl < G : DirectedGraph > DirectedGraph for ReversedGraph < G > { type Node = G :: Node ; fn num_nodes (& self) -> usize { self . inner . num_nodes () } }
};
}
