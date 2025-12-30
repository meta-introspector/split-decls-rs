// Generated macro for impl_309 (impl)
macro_rules! Depcrate_acyclicimpl_309 {
() => {
// Module: crate::acyclic
// Provides: {"impl_309"}
// Dependencies: {}
impl < G : Visitable + DataMapMut > DataMapMut for Acyclic < G > { fn node_weight_mut (& mut self , id : Self :: NodeId) -> Option < & mut Self :: NodeWeight > { self . inner_mut () . node_weight_mut (id) } fn edge_weight_mut (& mut self , id : Self :: EdgeId) -> Option < & mut Self :: EdgeWeight > { self . inner_mut () . edge_weight_mut (id) } }
};
}
