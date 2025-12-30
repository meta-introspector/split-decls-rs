// Generated macro for print_edge (function)
macro_rules! Depcrate_gv_parser_printerprint_edge {
() => {
// Module: crate::gv::parser::printer
// Provides: {"print_edge"}
// Dependencies: {}
fn print_edge (e : & ast :: EdgeStmt , indent : usize) { print_node_id (& e . from , indent + 1) ; for dest in & e . to { print_arrow (& dest . 1 , indent + 1) ; print_node_id (& dest . 0 , indent + 1) ; } print_attribute_list (& e . list , indent + 1) ; }
};
}
