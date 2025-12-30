// Generated macro for DepthFirstSearch (struct)
macro_rules! Depcrate_graph_iterateDepthFirstSearch {
() => {
// Module: crate::graph::iterate
// Provides: {"DepthFirstSearch"}
// Dependencies: {}
# [doc = " A \"depth-first search\" iterator for a directed graph."] pub struct DepthFirstSearch < G > where G : DirectedGraph + Successors , { graph : G , stack : Vec < G :: Node > , visited : DenseBitSet < G :: Node > , }
};
}
