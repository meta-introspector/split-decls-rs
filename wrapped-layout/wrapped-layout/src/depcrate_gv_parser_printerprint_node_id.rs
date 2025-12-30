// Generated macro for print_node_id (function)
macro_rules! Depcrate_gv_parser_printerprint_node_id {
() => {
// Module: crate::gv::parser::printer
// Provides: {"print_node_id"}
// Dependencies: {}
fn print_node_id (n : & ast :: NodeId , indent : usize) { print ! ("{}" , " " . repeat (indent)) ; if let Option :: Some (port) = & n . port { println ! ("{}:{}" , n . name , port) ; } else { println ! ("{}" , n . name) } }
};
}
