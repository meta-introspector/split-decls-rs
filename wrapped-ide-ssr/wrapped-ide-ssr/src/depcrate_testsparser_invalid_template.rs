// Generated macro for parser_invalid_template (function)
macro_rules! Depcrate_testsparser_invalid_template {
() => {
// Module: crate::tests
// Provides: {"parser_invalid_template"}
// Dependencies: {}
# [test] fn parser_invalid_template () { assert_eq ! (parse_error_text ("() ==>> )") , "Parse error: Not a valid Rust expression, type, item, path or pattern") ; }
};
}
