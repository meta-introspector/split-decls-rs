// Generated macro for impl_1140 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1140 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1140"}
// Dependencies: {}
impl < Ix : IndexType > WalkNeighbors < Ix > { # [doc = " Step to the next edge and its endpoint node in the walk for graph `g`."] # [doc = ""] # [doc = " The next node indices are always the others than the starting point"] # [doc = " where the `WalkNeighbors` value was created."] # [doc = " For an `Outgoing` walk, the target nodes,"] # [doc = " for an `Incoming` walk, the source nodes of the edge."] pub fn next < N , E , Ty : EdgeType > (& mut self , g : & StableGraph < N , E , Ty , Ix > ,) -> Option < (EdgeIndex < Ix > , NodeIndex < Ix >) > { self . inner . next (& g . g) } pub fn next_node < N , E , Ty : EdgeType > (& mut self , g : & StableGraph < N , E , Ty , Ix > ,) -> Option < NodeIndex < Ix > > { self . next (g) . map (| t | t . 1) } pub fn next_edge < N , E , Ty : EdgeType > (& mut self , g : & StableGraph < N , E , Ty , Ix > ,) -> Option < EdgeIndex < Ix > > { self . next (g) . map (| t | t . 0) } }
};
}
