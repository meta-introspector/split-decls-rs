// Generated macro for impl_1346 (impl)
macro_rules! Depcrate_matrix_graphimpl_1346 {
() => {
// Module: crate::matrix_graph
// Provides: {"impl_1346"}
// Dependencies: {}
impl < 'a , N , E : 'a , S : BuildHasher , Ty : EdgeType , Null : Nullable < Wrapped = E > , Ix : IndexType > IntoNodeIdentifiers for & 'a MatrixGraph < N , E , S , Ty , Null , Ix > { type NodeIdentifiers = NodeIdentifiers < 'a , Ix , S > ; fn node_identifiers (self) -> Self :: NodeIdentifiers { NodeIdentifiers :: new (self . nodes . iter_ids ()) } }
};
}
