// Generated macro for impl_1110 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1110 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1110"}
// Dependencies: {}
# [doc = " Index the `StableGraph` by `NodeIndex` to access node weights."] # [doc = ""] # [doc = " **Panics** if the node doesn't exist."] impl < N , E , Ty , Ix > Index < NodeIndex < Ix > > for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Output = N ; fn index (& self , index : NodeIndex < Ix >) -> & N { self . node_weight (index) . unwrap () } }
};
}
