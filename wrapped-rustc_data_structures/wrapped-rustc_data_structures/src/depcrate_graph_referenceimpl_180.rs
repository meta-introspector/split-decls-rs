// Generated macro for impl_180 (impl)
macro_rules! Depcrate_graph_referenceimpl_180 {
() => {
// Module: crate::graph::reference
// Provides: {"impl_180"}
// Dependencies: {}
impl < 'graph , G : DirectedGraph > DirectedGraph for & 'graph G { type Node = G :: Node ; fn num_nodes (& self) -> usize { (* * self) . num_nodes () } }
};
}
