// Generated macro for lex_ok (function)
macro_rules! Depcrate_testslex_ok {
() => {
// Module: crate::tests
// Provides: {"lex_ok"}
// Dependencies: {}
# [test] fn lex_ok () { for case in TestCase :: list ("lexer/ok") { let _guard = stdx :: panic_context :: enter (format ! ("{:?}" , case . rs)) ; let actual = lex (& case . text , infer_edition (& case . rs)) ; expect_file ! [case . rast] . assert_eq (& actual) } }
};
}
