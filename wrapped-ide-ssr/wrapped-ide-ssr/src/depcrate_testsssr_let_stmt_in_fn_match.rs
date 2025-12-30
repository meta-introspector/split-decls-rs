// Generated macro for ssr_let_stmt_in_fn_match (function)
macro_rules! Depcrate_testsssr_let_stmt_in_fn_match {
() => {
// Module: crate::tests
// Provides: {"ssr_let_stmt_in_fn_match"}
// Dependencies: {}
# [test] fn ssr_let_stmt_in_fn_match () { assert_matches ("let $a = 10;" , "fn main() { let x = 10; x }" , & ["let x = 10;"]) ; assert_matches ("let $a = $b;" , "fn main() { let x = 10; x }" , & ["let x = 10;"]) ; }
};
}
