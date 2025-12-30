// Generated macro for parser_undefined_placeholder_in_replacement (function)
macro_rules! Depcrate_testsparser_undefined_placeholder_in_replacement {
() => {
// Module: crate::tests
// Provides: {"parser_undefined_placeholder_in_replacement"}
// Dependencies: {}
# [test] fn parser_undefined_placeholder_in_replacement () { assert_eq ! (parse_error_text ("42 ==>> $a") , "Parse error: Replacement contains undefined placeholders: $a") ; }
};
}
