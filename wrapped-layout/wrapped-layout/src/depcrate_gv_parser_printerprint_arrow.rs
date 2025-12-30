// Generated macro for print_arrow (function)
macro_rules! Depcrate_gv_parser_printerprint_arrow {
() => {
// Module: crate::gv::parser::printer
// Provides: {"print_arrow"}
// Dependencies: {}
fn print_arrow (k : & ast :: ArrowKind , indent : usize) { print ! ("{}" , " " . repeat (indent)) ; match k { ast :: ArrowKind :: Arrow => { println ! ("->") ; } ast :: ArrowKind :: Line => { println ! ("--") ; } } }
};
}
