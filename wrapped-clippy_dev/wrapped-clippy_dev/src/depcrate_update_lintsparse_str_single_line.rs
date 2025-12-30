// Generated macro for parse_str_single_line (function)
macro_rules! Depcrate_update_lintsparse_str_single_line {
() => {
// Module: crate::update_lints
// Provides: {"parse_str_single_line"}
// Dependencies: {}
fn parse_str_single_line (path : & Path , s : & str) -> String { let value = parse_str_lit (s) ; assert ! (! value . contains ('\n') , "error parsing `{}`: `{s}` should be a single line string" , path . display () ,) ; value }
};
}
