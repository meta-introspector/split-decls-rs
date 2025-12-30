// Generated macro for impl_356 (impl)
macro_rules! Depcrate_adjimpl_356 {
() => {
// Module: crate::adj
// Provides: {"impl_356"}
// Dependencies: {}
impl < 'a , E , Ix : IndexType > IntoNeighbors for & 'a List < E , Ix > { type Neighbors = Neighbors < 'a , E , Ix > ; # [doc = " Returns an iterator of all nodes with an edge starting from `a`."] # [doc = " Panics if `a` is out of bounds."] # [doc = " Use [`List::edge_indices_from`] instead if you do not want to borrow the adjacency list while"] # [doc = " iterating."] # [track_caller] fn neighbors (self , a : NodeIndex < Ix >) -> Self :: Neighbors { let proj : fn (& WSuc < E , Ix >) -> NodeIndex < Ix > = | x | x . suc ; let iter = self . suc [a . index ()] . iter () . map (proj) ; Neighbors { iter } } }
};
}
