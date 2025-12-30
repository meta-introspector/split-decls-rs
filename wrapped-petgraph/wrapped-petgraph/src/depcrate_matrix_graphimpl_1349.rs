// Generated macro for impl_1349 (impl)
macro_rules! Depcrate_matrix_graphimpl_1349 {
() => {
// Module: crate::matrix_graph
// Provides: {"impl_1349"}
// Dependencies: {}
impl < 'a , N , E , Ty : EdgeType , Null : Nullable < Wrapped = E > , Ix : IndexType , S : BuildHasher + 'a > IntoNodeReferences for & 'a MatrixGraph < N , E , S , Ty , Null , Ix > { type NodeRef = (NodeIndex < Ix > , & 'a N) ; type NodeReferences = NodeReferences < 'a , N , Ix , S > ; fn node_references (self) -> Self :: NodeReferences { NodeReferences :: new (& self . nodes) } }
};
}
