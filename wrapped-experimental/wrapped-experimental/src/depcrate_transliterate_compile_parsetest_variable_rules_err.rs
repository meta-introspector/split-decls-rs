// Generated macro for test_variable_rules_err (function)
macro_rules! Depcrate_transliterate_compile_parsetest_variable_rules_err {
() => {
// Module: crate::transliterate::compile::parse
// Provides: {"test_variable_rules_err"}
// Dependencies: {}
# [test] fn test_variable_rules_err () { let sources = [r" $ my_var = a ;" , r" $my_var = a_2 ;" , r"$my_var 2 = [a-z] literal ;" , r"$my_var = [$doesnt_exist] ;" ,] ; for source in sources { if let Ok (rules) = parse (source) { panic ! ("Parsed invalid source {source:?}: {rules:?}") ; } } }
};
}
