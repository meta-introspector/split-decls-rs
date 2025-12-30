// Generated macro for impl_272 (impl)
macro_rules! Depcrate_dataimpl_272 {
() => {
// Module: crate::data
// Provides: {"impl_272"}
// Dependencies: {}
# [cfg (feature = "stable_graph")] impl < N , E , Ty , Ix > Build for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn add_node (& mut self , weight : Self :: NodeWeight) -> Self :: NodeId { self . add_node (weight) } fn add_edge (& mut self , a : Self :: NodeId , b : Self :: NodeId , weight : Self :: EdgeWeight ,) -> Option < Self :: EdgeId > { Some (self . add_edge (a , b , weight)) } fn update_edge (& mut self , a : Self :: NodeId , b : Self :: NodeId , weight : Self :: EdgeWeight ,) -> Self :: EdgeId { self . update_edge (a , b , weight) } }
};
}
