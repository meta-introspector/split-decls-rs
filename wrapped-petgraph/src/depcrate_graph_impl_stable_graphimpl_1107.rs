// Generated macro for impl_1107 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1107 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1107"}
// Dependencies: {}
impl < N , E , Ty , Ix > StableGraph < N , E , Ty , Ix > where Ix : IndexType , { # [doc = " Create a new `StableGraph` with estimated capacity."] pub fn with_capacity (nodes : usize , edges : usize) -> Self { StableGraph { g : Graph :: with_capacity (nodes , edges) , node_count : 0 , edge_count : 0 , free_node : NodeIndex :: end () , free_edge : EdgeIndex :: end () , } } }
};
}
