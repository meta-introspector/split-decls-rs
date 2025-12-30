// Generated macro for impl_1015 (impl)
macro_rules! Depcrate_graph_implimpl_1015 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_1015"}
// Dependencies: {}
impl < 'a , N : 'a , E : 'a , Ty , Ix > visit :: IntoEdgeReferences for & 'a Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type EdgeRef = EdgeReference < 'a , E , Ix > ; type EdgeReferences = EdgeReferences < 'a , E , Ix > ; fn edge_references (self) -> Self :: EdgeReferences { (* self) . edge_references () } }
};
}
