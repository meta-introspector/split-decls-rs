// Generated macro for impl_1136 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1136 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1136"}
// Dependencies: {}
impl < E , Ix > Neighbors < '_ , E , Ix > where Ix : IndexType , { # [doc = " Return a “walker” object that can be used to step through the"] # [doc = " neighbors and edges from the origin node."] # [doc = ""] # [doc = " Note: The walker does not borrow from the graph, this is to allow mixing"] # [doc = " edge walking with mutating the graph's weights."] pub fn detach (& self) -> WalkNeighbors < Ix > { WalkNeighbors { inner : super :: WalkNeighbors { skip_start : self . skip_start , next : self . next , } , } } }
};
}
