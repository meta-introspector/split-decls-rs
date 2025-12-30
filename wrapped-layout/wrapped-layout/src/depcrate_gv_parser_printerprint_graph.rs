// Generated macro for print_graph (function)
macro_rules! Depcrate_gv_parser_printerprint_graph {
() => {
// Module: crate::gv::parser::printer
// Provides: {"print_graph"}
// Dependencies: {}
fn print_graph (graph : & ast :: Graph , indent : usize) { print ! ("{}" , " " . repeat (indent)) ; println ! ("Graph: {}" , graph . name) ; for stmt in & graph . list . list { print_stmt (stmt , indent + 1) ; } }
};
}
