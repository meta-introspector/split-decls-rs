// Generated macro for impl_963 (impl)
macro_rules! Depcrate_graph_implimpl_963 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_963"}
// Dependencies: {}
impl < E , Ix > Neighbors < '_ , E , Ix > where Ix : IndexType , { # [doc = " Return a “walker” object that can be used to step through the"] # [doc = " neighbors and edges from the origin node."] # [doc = ""] # [doc = " Note: The walker does not borrow from the graph, this is to allow mixing"] # [doc = " edge walking with mutating the graph's weights."] pub fn detach (& self) -> WalkNeighbors < Ix > { WalkNeighbors { skip_start : self . skip_start , next : self . next , } } }
};
}
