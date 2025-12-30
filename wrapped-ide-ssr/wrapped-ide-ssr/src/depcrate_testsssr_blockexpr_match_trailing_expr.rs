// Generated macro for ssr_blockexpr_match_trailing_expr (function)
macro_rules! Depcrate_testsssr_blockexpr_match_trailing_expr {
() => {
// Module: crate::tests
// Provides: {"ssr_blockexpr_match_trailing_expr"}
// Dependencies: {}
# [test] fn ssr_blockexpr_match_trailing_expr () { assert_matches ("if $a() {$b;}" , "{
    if foo() {
        bar();
    }
}" , & ["if foo() {
        bar();
    }"] ,) ; }
};
}
