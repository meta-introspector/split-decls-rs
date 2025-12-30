// Generated macro for impl_1156 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1156 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1156"}
// Dependencies: {}
impl < 'a , N , E : 'a , Ty , Ix > visit :: IntoNeighborsDirected for & 'a StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type NeighborsDirected = Neighbors < 'a , E , Ix > ; fn neighbors_directed (self , n : NodeIndex < Ix > , d : Direction) -> Self :: NeighborsDirected { StableGraph :: neighbors_directed (self , n , d) } }
};
}
