// Generated macro for impl_1155 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1155 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1155"}
// Dependencies: {}
impl < 'a , N , E : 'a , Ty , Ix > visit :: IntoNeighbors for & 'a StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Neighbors = Neighbors < 'a , E , Ix > ; fn neighbors (self , n : Self :: NodeId) -> Self :: Neighbors { (* self) . neighbors (n) } }
};
}
