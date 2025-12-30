// Generated macro for parse_lint_multiple_path (function)
macro_rules! Depcrate_testsparse_lint_multiple_path {
() => {
// Module: crate::tests
// Provides: {"parse_lint_multiple_path"}
// Dependencies: {}
# [test] fn parse_lint_multiple_path () { create_default_session_globals_then (| | { assert_eq ! (parse_lint_and_tool_name ("clippy::foo::bar") , (Some (Symbol :: intern ("clippy")) , "foo::bar")) }) ; }
};
}
