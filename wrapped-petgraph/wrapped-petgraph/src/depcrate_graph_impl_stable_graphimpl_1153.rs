// Generated macro for impl_1153 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1153 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1153"}
// Dependencies: {}
impl < 'a , N , E , Ty , Ix > visit :: IntoNodeReferences for & 'a StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type NodeRef = (NodeIndex < Ix > , & 'a N) ; type NodeReferences = NodeReferences < 'a , N , Ix > ; fn node_references (self) -> Self :: NodeReferences { NodeReferences { iter : self . raw_nodes () . iter () . enumerate () , } } }
};
}
