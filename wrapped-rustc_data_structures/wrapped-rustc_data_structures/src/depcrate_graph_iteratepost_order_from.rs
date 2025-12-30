// Generated macro for post_order_from (function)
macro_rules! Depcrate_graph_iteratepost_order_from {
() => {
// Module: crate::graph::iterate
// Provides: {"post_order_from"}
// Dependencies: {}
pub fn post_order_from < G : DirectedGraph + Successors > (graph : & G , start_node : G :: Node ,) -> Vec < G :: Node > { post_order_from_to (graph , start_node , None) }
};
}
