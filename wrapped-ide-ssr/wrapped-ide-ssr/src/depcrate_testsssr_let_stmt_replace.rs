// Generated macro for ssr_let_stmt_replace (function)
macro_rules! Depcrate_testsssr_let_stmt_replace {
() => {
// Module: crate::tests
// Provides: {"ssr_let_stmt_replace"}
// Dependencies: {}
# [test] fn ssr_let_stmt_replace () { assert_ssr_transform ("let $a = $b; ==>> let $a = 11;" , "fn main() { let x = 10; x }" , expect ! [["fn main() { let x = 11; x }"]] ,) ; }
};
}
