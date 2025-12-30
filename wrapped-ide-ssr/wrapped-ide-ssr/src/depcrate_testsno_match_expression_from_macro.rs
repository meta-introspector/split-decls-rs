// Generated macro for no_match_expression_from_macro (function)
macro_rules! Depcrate_testsno_match_expression_from_macro {
() => {
// Module: crate::tests
// Provides: {"no_match_expression_from_macro"}
// Dependencies: {}
# [test] fn no_match_expression_from_macro () { assert_no_match ("$a.clone()" , r#"
            macro_rules! m1 {
                () => {42.clone()}
            }
            fn f1() {m1!()}
            "# ,) ; }
};
}
