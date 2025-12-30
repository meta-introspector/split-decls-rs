// Generated macro for no_match_split_expression (function)
macro_rules! Depcrate_testsno_match_split_expression {
() => {
// Module: crate::tests
// Provides: {"no_match_split_expression"}
// Dependencies: {}
# [test] fn no_match_split_expression () { assert_no_match ("$a.clone()" , r#"
            macro_rules! m1 {
                ($x:expr) => {$x.clone()}
            }
            fn f1() {m1!(42)}
            "# ,) ; }
};
}
