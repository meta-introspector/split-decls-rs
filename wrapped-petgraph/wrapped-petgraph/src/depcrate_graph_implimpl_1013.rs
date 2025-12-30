// Generated macro for impl_1013 (impl)
macro_rules! Depcrate_graph_implimpl_1013 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_1013"}
// Dependencies: {}
impl < 'a , N , E : 'a , Ty , Ix > visit :: IntoNeighbors for & 'a Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Neighbors = Neighbors < 'a , E , Ix > ; fn neighbors (self , n : NodeIndex < Ix >) -> Neighbors < 'a , E , Ix > { Graph :: neighbors (self , n) } }
};
}
