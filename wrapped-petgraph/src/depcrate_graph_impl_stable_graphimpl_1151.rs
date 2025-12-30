// Generated macro for impl_1151 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1151 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1151"}
// Dependencies: {}
impl < 'a , N , E : 'a , Ty , Ix > visit :: IntoNodeIdentifiers for & 'a StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type NodeIdentifiers = NodeIndices < 'a , N , Ix > ; fn node_identifiers (self) -> Self :: NodeIdentifiers { StableGraph :: node_indices (self) } }
};
}
