// Generated macro for test_dollar_crate_lhs_is_not_meta (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_dollar_crate_lhs_is_not_meta {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_dollar_crate_lhs_is_not_meta"}
// Dependencies: {}
# [test] fn test_dollar_crate_lhs_is_not_meta () { check (r#"
macro_rules! m {
    ($crate) => { err!(); };
    () => { ok!(); };
}
m!{}
"# , expect ! [[r#"
macro_rules! m {
    ($crate) => { err!(); };
    () => { ok!(); };
}
ok!();
"#]] ,) ; }
};
}
