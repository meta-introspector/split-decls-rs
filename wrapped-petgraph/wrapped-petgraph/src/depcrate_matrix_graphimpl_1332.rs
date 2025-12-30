// Generated macro for impl_1332 (impl)
macro_rules! Depcrate_matrix_graphimpl_1332 {
() => {
// Module: crate::matrix_graph
// Provides: {"impl_1332"}
// Dependencies: {}
# [doc = " Create a new empty `MatrixGraph`."] impl < N , E , S : BuildHasher + Default , Ty : EdgeType , Null : Nullable < Wrapped = E > , Ix : IndexType > Default for MatrixGraph < N , E , S , Ty , Null , Ix > { fn default () -> Self { Self :: with_capacity (0) } }
};
}
