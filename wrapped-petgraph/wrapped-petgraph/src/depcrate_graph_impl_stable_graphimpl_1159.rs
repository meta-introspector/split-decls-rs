// Generated macro for impl_1159 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1159 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1159"}
// Dependencies: {}
impl < N , E , Ty , Ix > visit :: EdgeIndexable for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn edge_bound (& self) -> usize { self . edge_references () . next_back () . map_or (0 , | edge | edge . id () . index () + 1) } fn to_index (& self , ix : EdgeIndex < Ix >) -> usize { ix . index () } fn from_index (& self , ix : usize) -> Self :: EdgeId { EdgeIndex :: new (ix) } }
};
}
