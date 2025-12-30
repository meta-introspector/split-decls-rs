// Generated macro for impl_1111 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1111 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1111"}
// Dependencies: {}
# [doc = " Index the `StableGraph` by `NodeIndex` to access node weights."] # [doc = ""] # [doc = " **Panics** if the node doesn't exist."] impl < N , E , Ty , Ix > IndexMut < NodeIndex < Ix > > for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn index_mut (& mut self , index : NodeIndex < Ix >) -> & mut N { self . node_weight_mut (index) . unwrap () } }
};
}
