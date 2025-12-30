// Generated macro for impl_984 (impl)
macro_rules! Depcrate_graph_implimpl_984 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_984"}
// Dependencies: {}
# [doc = " Index the `Graph` by `NodeIndex` to access node weights."] # [doc = ""] # [doc = " **Panics** if the node doesn't exist."] impl < N , E , Ty , Ix > IndexMut < NodeIndex < Ix > > for Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn index_mut (& mut self , index : NodeIndex < Ix >) -> & mut N { & mut self . nodes [index . index ()] . weight } }
};
}
