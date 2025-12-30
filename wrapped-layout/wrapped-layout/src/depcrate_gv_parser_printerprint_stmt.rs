// Generated macro for print_stmt (function)
macro_rules! Depcrate_gv_parser_printerprint_stmt {
() => {
// Module: crate::gv::parser::printer
// Provides: {"print_stmt"}
// Dependencies: {}
fn print_stmt (stmt : & ast :: Stmt , indent : usize) { match stmt { ast :: Stmt :: Edge (e) => { print_edge (e , indent) ; } ast :: Stmt :: Node (n) => { print_node (n , indent) ; } ast :: Stmt :: Attribute (a) => { print_att (a , indent) ; } ast :: Stmt :: SubGraph (g) => { print_graph (g , indent) ; } } }
};
}
