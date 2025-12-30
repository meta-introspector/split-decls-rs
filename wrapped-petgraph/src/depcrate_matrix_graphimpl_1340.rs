// Generated macro for impl_1340 (impl)
macro_rules! Depcrate_matrix_graphimpl_1340 {
() => {
// Module: crate::matrix_graph
// Provides: {"impl_1340"}
// Dependencies: {}
# [doc = " Index the `MatrixGraph` by `NodeIndex` pair to access edge weights."] # [doc = ""] # [doc = " Also available with indexing syntax: `&mut graph[e]`."] # [doc = ""] # [doc = " **Panics** if no edge exists between `a` and `b`."] impl < N , E , S : BuildHasher , Ty : EdgeType , Null : Nullable < Wrapped = E > , Ix : IndexType > IndexMut < (NodeIndex < Ix > , NodeIndex < Ix >) > for MatrixGraph < N , E , S , Ty , Null , Ix > { fn index_mut (& mut self , (ax , bx) : (NodeIndex < Ix > , NodeIndex < Ix >)) -> & mut E { self . edge_weight_mut (ax , bx) } }
};
}
