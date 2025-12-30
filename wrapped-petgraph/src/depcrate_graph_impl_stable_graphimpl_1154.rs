// Generated macro for impl_1154 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1154 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1154"}
// Dependencies: {}
impl < N , E , Ty , Ix > visit :: NodeIndexable for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { # [doc = " Return an upper bound of the node indices in the graph"] fn node_bound (& self) -> usize { self . node_indices () . next_back () . map_or (0 , | i | i . index () + 1) } fn to_index (& self , ix : NodeIndex < Ix >) -> usize { ix . index () } fn from_index (& self , ix : usize) -> Self :: NodeId { NodeIndex :: new (ix) } }
};
}
