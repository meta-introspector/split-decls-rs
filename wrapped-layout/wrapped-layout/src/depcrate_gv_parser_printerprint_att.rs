// Generated macro for print_att (function)
macro_rules! Depcrate_gv_parser_printerprint_att {
() => {
// Module: crate::gv::parser::printer
// Provides: {"print_att"}
// Dependencies: {}
fn print_att (att : & ast :: AttrStmt , indent : usize) { print ! ("{}" , " " . repeat (indent)) ; match att . target { ast :: AttrStmtTarget :: Graph => { println ! ("Attribute Graph:") ; } ast :: AttrStmtTarget :: Node => { println ! ("Attribute Node:") ; } ast :: AttrStmtTarget :: Edge => { println ! ("Attribute Edge:") ; } } print_attribute_list (& att . list , indent + 1) ; }
};
}
