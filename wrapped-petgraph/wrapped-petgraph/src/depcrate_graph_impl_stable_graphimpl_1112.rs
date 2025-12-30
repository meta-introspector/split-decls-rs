// Generated macro for impl_1112 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1112 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1112"}
// Dependencies: {}
# [doc = " Index the `StableGraph` by `EdgeIndex` to access edge weights."] # [doc = ""] # [doc = " **Panics** if the edge doesn't exist."] impl < N , E , Ty , Ix > Index < EdgeIndex < Ix > > for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Output = E ; fn index (& self , index : EdgeIndex < Ix >) -> & E { self . edge_weight (index) . unwrap () } }
};
}
