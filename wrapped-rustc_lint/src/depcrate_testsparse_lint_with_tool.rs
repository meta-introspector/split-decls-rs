// Generated macro for parse_lint_with_tool (function)
macro_rules! Depcrate_testsparse_lint_with_tool {
() => {
// Module: crate::tests
// Provides: {"parse_lint_with_tool"}
// Dependencies: {}
# [test] fn parse_lint_with_tool () { create_default_session_globals_then (| | { assert_eq ! (parse_lint_and_tool_name ("clippy::foo") , (Some (Symbol :: intern ("clippy")) , "foo")) }) ; }
};
}
