// Generated macro for impl_269 (impl)
macro_rules! Depcrate_dataimpl_269 {
() => {
// Module: crate::data
// Provides: {"impl_269"}
// Dependencies: {}
# [cfg (feature = "stable_graph")] impl < N , E , Ty , Ix > DataMap for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn node_weight (& self , id : Self :: NodeId) -> Option < & Self :: NodeWeight > { self . node_weight (id) } fn edge_weight (& self , id : Self :: EdgeId) -> Option < & Self :: EdgeWeight > { self . edge_weight (id) } }
};
}
