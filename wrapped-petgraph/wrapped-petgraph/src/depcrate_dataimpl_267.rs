// Generated macro for impl_267 (impl)
macro_rules! Depcrate_dataimpl_267 {
() => {
// Module: crate::data
// Provides: {"impl_267"}
// Dependencies: {}
impl < N , E , Ty , Ix > DataMap for Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn node_weight (& self , id : Self :: NodeId) -> Option < & Self :: NodeWeight > { self . node_weight (id) } fn edge_weight (& self , id : Self :: EdgeId) -> Option < & Self :: EdgeWeight > { self . edge_weight (id) } }
};
}
