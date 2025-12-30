// Generated macro for impl_174 (impl)
macro_rules! Depcrate_visit_filterimpl_174 {
() => {
// Module: crate::visit::filter
// Provides: {"impl_174"}
// Dependencies: {}
impl < 'a , G , F > IntoNeighborsDirected for & 'a EdgeFiltered < G , F > where G : IntoEdgesDirected , F : FilterEdge < G :: EdgeRef > , { type NeighborsDirected = EdgeFilteredNeighborsDirected < 'a , G , F > ; fn neighbors_directed (self , n : G :: NodeId , dir : Direction) -> Self :: NeighborsDirected { EdgeFilteredNeighborsDirected { iter : self . 0 . edges_directed (n , dir) , f : & self . 1 , from : n , } } }
};
}
