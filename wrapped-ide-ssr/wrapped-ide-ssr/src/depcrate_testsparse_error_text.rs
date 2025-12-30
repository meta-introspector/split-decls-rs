// Generated macro for parse_error_text (function)
macro_rules! Depcrate_testsparse_error_text {
() => {
// Module: crate::tests
// Provides: {"parse_error_text"}
// Dependencies: {}
fn parse_error_text (query : & str) -> String { format ! ("{}" , query . parse ::< SsrRule > () . unwrap_err ()) }
};
}
