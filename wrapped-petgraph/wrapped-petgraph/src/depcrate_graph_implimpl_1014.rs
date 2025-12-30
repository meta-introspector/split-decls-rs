// Generated macro for impl_1014 (impl)
macro_rules! Depcrate_graph_implimpl_1014 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_1014"}
// Dependencies: {}
impl < 'a , N , E : 'a , Ty , Ix > visit :: IntoNeighborsDirected for & 'a Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type NeighborsDirected = Neighbors < 'a , E , Ix > ; fn neighbors_directed (self , n : NodeIndex < Ix > , d : Direction) -> Neighbors < 'a , E , Ix > { Graph :: neighbors_directed (self , n , d) } }
};
}
