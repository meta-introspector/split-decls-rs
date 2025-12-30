// Generated macro for impl_1011 (impl)
macro_rules! Depcrate_graph_implimpl_1011 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_1011"}
// Dependencies: {}
impl < N , E , Ty , Ix > visit :: NodeIndexable for Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { # [inline] fn node_bound (& self) -> usize { self . node_count () } # [inline] fn to_index (& self , ix : NodeIndex < Ix >) -> usize { ix . index () } # [inline] fn from_index (& self , ix : usize) -> Self :: NodeId { NodeIndex :: new (ix) } }
};
}
