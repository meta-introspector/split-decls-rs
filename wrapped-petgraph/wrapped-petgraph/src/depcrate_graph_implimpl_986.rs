// Generated macro for impl_986 (impl)
macro_rules! Depcrate_graph_implimpl_986 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_986"}
// Dependencies: {}
# [doc = " Index the `Graph` by `EdgeIndex` to access edge weights."] # [doc = ""] # [doc = " **Panics** if the edge doesn't exist."] impl < N , E , Ty , Ix > IndexMut < EdgeIndex < Ix > > for Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn index_mut (& mut self , index : EdgeIndex < Ix >) -> & mut E { & mut self . edges [index . index ()] . weight } }
};
}
