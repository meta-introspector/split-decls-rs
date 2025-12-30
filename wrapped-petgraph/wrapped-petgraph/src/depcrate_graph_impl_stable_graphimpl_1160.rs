// Generated macro for impl_1160 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1160 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1160"}
// Dependencies: {}
impl < 'a , N , E , Ty , Ix > visit :: IntoEdgesDirected for & 'a StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type EdgesDirected = Edges < 'a , E , Ty , Ix > ; fn edges_directed (self , a : Self :: NodeId , dir : Direction) -> Self :: EdgesDirected { self . edges_directed (a , dir) } }
};
}
