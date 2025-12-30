// Generated macro for impl_268 (impl)
macro_rules! Depcrate_dataimpl_268 {
() => {
// Module: crate::data
// Provides: {"impl_268"}
// Dependencies: {}
impl < N , E , Ty , Ix > DataMapMut for Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn node_weight_mut (& mut self , id : Self :: NodeId) -> Option < & mut Self :: NodeWeight > { self . node_weight_mut (id) } fn edge_weight_mut (& mut self , id : Self :: EdgeId) -> Option < & mut Self :: EdgeWeight > { self . edge_weight_mut (id) } }
};
}
