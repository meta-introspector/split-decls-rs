// Generated macro for impl_968 (impl)
macro_rules! Depcrate_graph_implimpl_968 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_968"}
// Dependencies: {}
impl < 'a , N , E , Ty , Ix > visit :: IntoEdgesDirected for & 'a Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type EdgesDirected = Edges < 'a , E , Ty , Ix > ; fn edges_directed (self , a : Self :: NodeId , dir : Direction) -> Self :: EdgesDirected { self . edges_directed (a , dir) } }
};
}
