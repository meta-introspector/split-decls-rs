// Generated macro for trace_argument_code_string (function)
macro_rules! Depcrate_render_testtrace_argument_code_string {
() => {
// Module: crate::render::test
// Provides: {"trace_argument_code_string"}
// Dependencies: {}
fn trace_argument_code_string (arg_name : & str) -> String { let arg_name = ident (arg_name) ; let statement : Stmt = parse_quote ! { println ! ("{} = {:?}" , stringify ! (# arg_name) ,# arg_name) ; } ; statement . display_code () }
};
}
