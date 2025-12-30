// Generated macro for ref_argument_code_string (function)
macro_rules! Depcrate_testref_argument_code_string {
() => {
// Module: crate::test
// Provides: {"ref_argument_code_string"}
// Dependencies: {}
pub (crate) fn ref_argument_code_string (arg_name : & str) -> String { let arg_name = ident (arg_name) ; let statement : Expr = parse_quote ! { &# arg_name } ; statement . display_code () }
};
}
