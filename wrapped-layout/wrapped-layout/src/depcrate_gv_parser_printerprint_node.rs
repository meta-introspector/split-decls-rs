// Generated macro for print_node (function)
macro_rules! Depcrate_gv_parser_printerprint_node {
() => {
// Module: crate::gv::parser::printer
// Provides: {"print_node"}
// Dependencies: {}
fn print_node (n : & ast :: NodeStmt , indent : usize) { print ! ("Node {}" , " " . repeat (indent)) ; print_node_id (& n . id , indent + 1) ; print_attribute_list (& n . list , indent + 1) ; }
};
}
