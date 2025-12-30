// Generated macro for impl_173 (impl)
macro_rules! Depcrate_visit_filterimpl_173 {
() => {
// Module: crate::visit::filter
// Provides: {"impl_173"}
// Dependencies: {}
impl < 'a , G , F > IntoNeighbors for & 'a EdgeFiltered < G , F > where G : IntoEdges , F : FilterEdge < G :: EdgeRef > , { type Neighbors = EdgeFilteredNeighbors < 'a , G , F > ; fn neighbors (self , n : G :: NodeId) -> Self :: Neighbors { EdgeFilteredNeighbors { iter : self . 0 . edges (n) , f : & self . 1 , } } }
};
}
