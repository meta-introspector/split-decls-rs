// Generated macro for parser_repeated_name (function)
macro_rules! Depcrate_testsparser_repeated_name {
() => {
// Module: crate::tests
// Provides: {"parser_repeated_name"}
// Dependencies: {}
# [test] fn parser_repeated_name () { assert_eq ! (parse_error_text ("foo($a, $a) ==>>") , "Parse error: Placeholder `$a` repeats more than once") ; }
};
}
