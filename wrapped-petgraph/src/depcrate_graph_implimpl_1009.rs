// Generated macro for impl_1009 (impl)
macro_rules! Depcrate_graph_implimpl_1009 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_1009"}
// Dependencies: {}
impl < 'a , N , E : 'a , Ty , Ix > visit :: IntoNodeIdentifiers for & 'a Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type NodeIdentifiers = NodeIndices < Ix > ; fn node_identifiers (self) -> NodeIndices < Ix > { Graph :: node_indices (self) } }
};
}
