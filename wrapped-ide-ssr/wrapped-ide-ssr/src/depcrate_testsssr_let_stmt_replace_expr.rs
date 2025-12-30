// Generated macro for ssr_let_stmt_replace_expr (function)
macro_rules! Depcrate_testsssr_let_stmt_replace_expr {
() => {
// Module: crate::tests
// Provides: {"ssr_let_stmt_replace_expr"}
// Dependencies: {}
# [test] fn ssr_let_stmt_replace_expr () { assert_ssr_transform ("let $a = $b; ==>> $b" , "fn main() { let x = 10; }" , expect ! [["fn main() { 10 }"]] ,) ; }
};
}
