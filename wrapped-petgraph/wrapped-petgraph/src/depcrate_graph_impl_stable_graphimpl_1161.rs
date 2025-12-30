// Generated macro for impl_1161 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1161 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1161"}
// Dependencies: {}
impl < 'a , N : 'a , E : 'a , Ty , Ix > visit :: IntoEdgeReferences for & 'a StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type EdgeRef = EdgeReference < 'a , E , Ix > ; type EdgeReferences = EdgeReferences < 'a , E , Ix > ; # [doc = " Create an iterator over all edges in the graph, in indexed order."] # [doc = ""] # [doc = " Iterator element type is `EdgeReference<E, Ix>`."] fn edge_references (self) -> Self :: EdgeReferences { EdgeReferences { iter : self . g . edges . iter () . enumerate () , } } }
};
}
