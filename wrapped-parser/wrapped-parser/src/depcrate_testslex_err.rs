// Generated macro for lex_err (function)
macro_rules! Depcrate_testslex_err {
() => {
// Module: crate::tests
// Provides: {"lex_err"}
// Dependencies: {}
# [test] fn lex_err () { for case in TestCase :: list ("lexer/err") { let _guard = stdx :: panic_context :: enter (format ! ("{:?}" , case . rs)) ; let actual = lex (& case . text , infer_edition (& case . rs)) ; expect_file ! [case . rast] . assert_eq (& actual) } }
};
}
