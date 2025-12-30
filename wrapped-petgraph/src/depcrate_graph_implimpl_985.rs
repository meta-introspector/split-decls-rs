// Generated macro for impl_985 (impl)
macro_rules! Depcrate_graph_implimpl_985 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_985"}
// Dependencies: {}
# [doc = " Index the `Graph` by `EdgeIndex` to access edge weights."] # [doc = ""] # [doc = " **Panics** if the edge doesn't exist."] impl < N , E , Ty , Ix > Index < EdgeIndex < Ix > > for Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Output = E ; fn index (& self , index : EdgeIndex < Ix >) -> & E { & self . edges [index . index ()] . weight } }
};
}
