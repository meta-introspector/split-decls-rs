// Generated macro for impl_200 (impl)
macro_rules! Depcrate_visit_reversedimpl_200 {
() => {
// Module: crate::visit::reversed
// Provides: {"impl_200"}
// Dependencies: {}
impl < G > IntoNeighbors for Reversed < G > where G : IntoNeighborsDirected , { type Neighbors = G :: NeighborsDirected ; fn neighbors (self , n : G :: NodeId) -> G :: NeighborsDirected { self . 0 . neighbors_directed (n , Incoming) } }
};
}
