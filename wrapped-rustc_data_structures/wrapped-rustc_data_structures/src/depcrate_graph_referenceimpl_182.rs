// Generated macro for impl_182 (impl)
macro_rules! Depcrate_graph_referenceimpl_182 {
() => {
// Module: crate::graph::reference
// Provides: {"impl_182"}
// Dependencies: {}
impl < 'graph , G : Successors > Successors for & 'graph G { fn successors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > { (* * self) . successors (node) } }
};
}
