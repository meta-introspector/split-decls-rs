// Generated macro for impl_149 (impl)
macro_rules! Depcrate_visit_filterimpl_149 {
() => {
// Module: crate::visit::filter
// Provides: {"impl_149"}
// Dependencies: {}
impl < 'a , G , F > IntoNeighborsDirected for & 'a NodeFiltered < G , F > where G : IntoNeighborsDirected , F : FilterNode < G :: NodeId > , { type NeighborsDirected = NodeFilteredNeighbors < 'a , G :: NeighborsDirected , F > ; fn neighbors_directed (self , n : G :: NodeId , dir : Direction) -> Self :: NeighborsDirected { NodeFilteredNeighbors { include_source : self . 1 . include_node (n) , iter : self . 0 . neighbors_directed (n , dir) , f : & self . 1 , } } }
};
}
