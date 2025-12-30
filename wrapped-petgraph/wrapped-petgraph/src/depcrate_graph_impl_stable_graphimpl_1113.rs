// Generated macro for impl_1113 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1113 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1113"}
// Dependencies: {}
# [doc = " Index the `StableGraph` by `EdgeIndex` to access edge weights."] # [doc = ""] # [doc = " **Panics** if the edge doesn't exist."] impl < N , E , Ty , Ix > IndexMut < EdgeIndex < Ix > > for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn index_mut (& mut self , index : EdgeIndex < Ix >) -> & mut E { self . edge_weight_mut (index) . unwrap () } }
};
}
