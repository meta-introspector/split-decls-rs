// Generated macro for impl_1157 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1157 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1157"}
// Dependencies: {}
impl < 'a , N , E , Ty , Ix > visit :: IntoEdges for & 'a StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Edges = Edges < 'a , E , Ty , Ix > ; fn edges (self , a : Self :: NodeId) -> Self :: Edges { self . edges (a) } }
};
}
