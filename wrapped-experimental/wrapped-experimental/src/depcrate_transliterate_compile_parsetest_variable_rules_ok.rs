// Generated macro for test_variable_rules_ok (function)
macro_rules! Depcrate_transliterate_compile_parsetest_variable_rules_ok {
() => {
// Module: crate::transliterate::compile::parse
// Provides: {"test_variable_rules_ok"}
// Dependencies: {}
# [test] fn test_variable_rules_ok () { let sources = [r" $my_var = [a-z] ;" , r"$my_var = äüöÜ ;" , r"$my_var = [a-z] literal ; $other_var = [A-Z] [b-z];" , r"$my_var = [a-z] ; $other_var = [A-Z] [b-z];" , r"$my_var = [a-z] ; $other_var = $my_var + $2222;" , r"$my_var = [a-z] ; $other_var = $my_var \+\ \$2222 \\ 'hello\';" , r"
        $innerMinus = '-' ;
        $minus = $innerMinus ;
        $good_set = [a $minus z] ;
        " ,] ; for source in sources { parse (source) . map_err (| e | e . explain (source)) . unwrap () ; } }
};
}
