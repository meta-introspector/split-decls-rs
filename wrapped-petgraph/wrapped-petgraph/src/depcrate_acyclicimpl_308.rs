// Generated macro for impl_308 (impl)
macro_rules! Depcrate_acyclicimpl_308 {
() => {
// Module: crate::acyclic
// Provides: {"impl_308"}
// Dependencies: {}
impl < G : Visitable + DataMap > DataMap for Acyclic < G > { fn node_weight (& self , id : Self :: NodeId) -> Option < & Self :: NodeWeight > { self . inner () . node_weight (id) } fn edge_weight (& self , id : Self :: EdgeId) -> Option < & Self :: EdgeWeight > { self . inner () . edge_weight (id) } }
};
}
