// Generated macro for impl_228 (impl)
macro_rules! Depcrate_visit_undirected_adaptorimpl_228 {
() => {
// Module: crate::visit::undirected_adaptor
// Provides: {"impl_228"}
// Dependencies: {}
impl < G > IntoNeighbors for UndirectedAdaptor < G > where G : IntoNeighborsDirected , { type Neighbors = core :: iter :: Chain < G :: NeighborsDirected , G :: NeighborsDirected > ; fn neighbors (self , n : G :: NodeId) -> Self :: Neighbors { self . 0 . neighbors_directed (n , Direction :: Incoming) . chain (self . 0 . neighbors_directed (n , Direction :: Outgoing)) } }
};
}
