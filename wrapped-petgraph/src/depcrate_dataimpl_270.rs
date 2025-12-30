// Generated macro for impl_270 (impl)
macro_rules! Depcrate_dataimpl_270 {
() => {
// Module: crate::data
// Provides: {"impl_270"}
// Dependencies: {}
# [cfg (feature = "stable_graph")] impl < N , E , Ty , Ix > DataMapMut for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn node_weight_mut (& mut self , id : Self :: NodeId) -> Option < & mut Self :: NodeWeight > { self . node_weight_mut (id) } fn edge_weight_mut (& mut self , id : Self :: EdgeId) -> Option < & mut Self :: EdgeWeight > { self . edge_weight_mut (id) } }
};
}
