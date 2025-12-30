// Generated macro for is_small_path_graph (function)
macro_rules! Depcrate_graph_dominatorsis_small_path_graph {
() => {
// Module: crate::graph::dominators
// Provides: {"is_small_path_graph"}
// Dependencies: {}
fn is_small_path_graph < G : ControlFlowGraph > (g : & G) -> bool { if g . start_node () . index () != 0 { return false ; } if g . num_nodes () == 1 { return true ; } if g . num_nodes () == 2 { return g . successors (g . start_node ()) . any (| n | n . index () == 1) ; } false }
};
}
