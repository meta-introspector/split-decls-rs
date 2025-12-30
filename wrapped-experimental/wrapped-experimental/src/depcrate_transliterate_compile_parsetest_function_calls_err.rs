// Generated macro for test_function_calls_err (function)
macro_rules! Depcrate_transliterate_compile_parsetest_function_calls_err {
() => {
// Module: crate::transliterate::compile::parse
// Provides: {"test_function_calls_err"}
// Dependencies: {}
# [test] fn test_function_calls_err () { let sources = [r"$fn = &[a-z]($var literal 'quoted literal' $1) ;" , r"$fn = &[a-z] ($var literal 'quoted literal' $1) ;" , r"$fn = &($var literal 'quoted literal' $1) ;" ,] ; for source in sources { if let Ok (rules) = parse (source) { panic ! ("Parsed invalid source {source:?}: {rules:?}") ; } } }
};
}
