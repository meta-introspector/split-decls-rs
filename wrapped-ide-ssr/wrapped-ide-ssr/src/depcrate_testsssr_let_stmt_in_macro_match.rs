// Generated macro for ssr_let_stmt_in_macro_match (function)
macro_rules! Depcrate_testsssr_let_stmt_in_macro_match {
() => {
// Module: crate::tests
// Provides: {"ssr_let_stmt_in_macro_match"}
// Dependencies: {}
# [test] fn ssr_let_stmt_in_macro_match () { assert_matches ("let a = 0" , r#"
            macro_rules! m1 { ($a:stmt) => {$a}; }
            fn f() {m1!{ let a = 0 };}"# , & ["leta=0"] ,) ; }
};
}
