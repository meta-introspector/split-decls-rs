// Generated macro for impl_1335 (impl)
macro_rules! Depcrate_matrix_graphimpl_1335 {
() => {
// Module: crate::matrix_graph
// Provides: {"impl_1335"}
// Dependencies: {}
# [doc = " Index the `MatrixGraph` by `NodeIndex` to access node weights."] # [doc = ""] # [doc = " **Panics** if the node doesn't exist."] impl < N , E , S : BuildHasher , Ty : EdgeType , Null : Nullable < Wrapped = E > , Ix : IndexType > Index < NodeIndex < Ix > > for MatrixGraph < N , E , S , Ty , Null , Ix > { type Output = N ; fn index (& self , ax : NodeIndex < Ix >) -> & N { self . node_weight (ax) } }
};
}
