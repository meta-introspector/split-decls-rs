// Generated macro for impl_983 (impl)
macro_rules! Depcrate_graph_implimpl_983 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_983"}
// Dependencies: {}
# [doc = " Index the `Graph` by `NodeIndex` to access node weights."] # [doc = ""] # [doc = " **Panics** if the node doesn't exist."] impl < N , E , Ty , Ix > Index < NodeIndex < Ix > > for Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Output = N ; fn index (& self , index : NodeIndex < Ix >) -> & N { & self . nodes [index . index ()] . weight } }
};
}
