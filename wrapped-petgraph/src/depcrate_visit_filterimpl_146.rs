// Generated macro for impl_146 (impl)
macro_rules! Depcrate_visit_filterimpl_146 {
() => {
// Module: crate::visit::filter
// Provides: {"impl_146"}
// Dependencies: {}
impl < 'a , G , F > IntoNeighbors for & 'a NodeFiltered < G , F > where G : IntoNeighbors , F : FilterNode < G :: NodeId > , { type Neighbors = NodeFilteredNeighbors < 'a , G :: Neighbors , F > ; fn neighbors (self , n : G :: NodeId) -> Self :: Neighbors { NodeFilteredNeighbors { include_source : self . 1 . include_node (n) , iter : self . 0 . neighbors (n) , f : & self . 1 , } } }
};
}
