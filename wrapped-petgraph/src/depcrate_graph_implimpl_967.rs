// Generated macro for impl_967 (impl)
macro_rules! Depcrate_graph_implimpl_967 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_967"}
// Dependencies: {}
impl < 'a , N , E , Ty , Ix > visit :: IntoEdges for & 'a Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Edges = Edges < 'a , E , Ty , Ix > ; fn edges (self , a : Self :: NodeId) -> Self :: Edges { self . edges (a) } }
};
}
