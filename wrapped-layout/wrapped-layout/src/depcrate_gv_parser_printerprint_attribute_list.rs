// Generated macro for print_attribute_list (function)
macro_rules! Depcrate_gv_parser_printerprint_attribute_list {
() => {
// Module: crate::gv::parser::printer
// Provides: {"print_attribute_list"}
// Dependencies: {}
fn print_attribute_list (ll : & ast :: AttributeList , indent : usize) { for (i , att) in ll . list . iter () . enumerate () { print_attribute (& att . 0 , & att . 1 , indent , i) ; } }
};
}
