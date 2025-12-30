// Generated macro for impl_1028 (impl)
macro_rules! Depcrate_graph_implimpl_1028 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_1028"}
// Dependencies: {}
impl < N , E , Ty , Ix > visit :: EdgeIndexable for Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn edge_bound (& self) -> usize { self . edge_count () } fn to_index (& self , ix : EdgeIndex < Ix >) -> usize { ix . index () } fn from_index (& self , ix : usize) -> Self :: EdgeId { EdgeIndex :: new (ix) } }
};
}
