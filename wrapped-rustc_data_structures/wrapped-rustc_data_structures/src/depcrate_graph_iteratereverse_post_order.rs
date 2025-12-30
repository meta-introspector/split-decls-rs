// Generated macro for reverse_post_order (function)
macro_rules! Depcrate_graph_iteratereverse_post_order {
() => {
// Module: crate::graph::iterate
// Provides: {"reverse_post_order"}
// Dependencies: {}
pub fn reverse_post_order < G : DirectedGraph + Successors > (graph : & G , start_node : G :: Node ,) -> Vec < G :: Node > { let mut vec = post_order_from (graph , start_node) ; vec . reverse () ; vec }
};
}
