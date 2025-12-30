// Generated macro for impl_201 (impl)
macro_rules! Depcrate_visit_reversedimpl_201 {
() => {
// Module: crate::visit::reversed
// Provides: {"impl_201"}
// Dependencies: {}
impl < G > IntoNeighborsDirected for Reversed < G > where G : IntoNeighborsDirected , { type NeighborsDirected = G :: NeighborsDirected ; fn neighbors_directed (self , n : G :: NodeId , d : Direction) -> G :: NeighborsDirected { self . 0 . neighbors_directed (n , d . opposite ()) } }
};
}
