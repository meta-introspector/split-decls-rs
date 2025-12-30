// Generated macro for impl_1017 (impl)
macro_rules! Depcrate_graph_implimpl_1017 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_1017"}
// Dependencies: {}
impl < 'a , N , E , Ty , Ix > visit :: IntoNodeReferences for & 'a Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type NodeRef = (NodeIndex < Ix > , & 'a N) ; type NodeReferences = NodeReferences < 'a , N , Ix > ; fn node_references (self) -> Self :: NodeReferences { NodeReferences { iter : self . nodes . iter () . enumerate () , } } }
};
}
