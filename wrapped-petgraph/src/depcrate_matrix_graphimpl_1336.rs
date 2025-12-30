// Generated macro for impl_1336 (impl)
macro_rules! Depcrate_matrix_graphimpl_1336 {
() => {
// Module: crate::matrix_graph
// Provides: {"impl_1336"}
// Dependencies: {}
# [doc = " Index the `MatrixGraph` by `NodeIndex` to access node weights."] # [doc = ""] # [doc = " **Panics** if the node doesn't exist."] impl < N , E , S : BuildHasher , Ty : EdgeType , Null : Nullable < Wrapped = E > , Ix : IndexType > IndexMut < NodeIndex < Ix > > for MatrixGraph < N , E , S , Ty , Null , Ix > { fn index_mut (& mut self , ax : NodeIndex < Ix >) -> & mut N { self . node_weight_mut (ax) } }
};
}
