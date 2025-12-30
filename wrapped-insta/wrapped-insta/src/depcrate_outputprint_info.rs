// Generated macro for print_info (function)
macro_rules! Depcrate_outputprint_info {
() => {
// Module: crate::output
// Provides: {"print_info"}
// Dependencies: {}
fn print_info (metadata : & MetaData) { let width = term_width () ; if let Some (expr) = metadata . expression () { println ! ("Expression: {}" , style (format_rust_expression (expr))) ; print_line (width) ; } if let Some (descr) = metadata . description () { println ! ("{descr}") ; print_line (width) ; } if let Some (info) = metadata . private_info () { let out = yaml :: to_string (info) ; println ! ("{}" , out . trim () . strip_prefix ("---") . unwrap () . trim_start ()) ; print_line (width) ; } }
};
}
