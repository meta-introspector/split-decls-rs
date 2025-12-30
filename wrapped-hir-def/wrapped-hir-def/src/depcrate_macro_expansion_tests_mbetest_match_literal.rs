// Generated macro for test_match_literal (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_match_literal {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_match_literal"}
// Dependencies: {}
# [test] fn test_match_literal () { check (r#"
macro_rules! m {
    ('(') => { fn l_paren() {} }
}
m!['('];
"# , expect ! [[r#"
macro_rules! m {
    ('(') => { fn l_paren() {} }
}
fn l_paren() {}
"#]] ,) ; }
};
}
