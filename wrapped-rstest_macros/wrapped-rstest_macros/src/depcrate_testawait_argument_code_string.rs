// Generated macro for await_argument_code_string (function)
macro_rules! Depcrate_testawait_argument_code_string {
() => {
// Module: crate::test
// Provides: {"await_argument_code_string"}
// Dependencies: {}
pub (crate) fn await_argument_code_string (arg_name : & str) -> String { let arg_name = ident (arg_name) ; let statement : Stmt = parse_quote ! { let # arg_name = # arg_name . await ; } ; statement . display_code () }
};
}
