// Generated macro for test_expr_with_attr (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_expr_with_attr {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_expr_with_attr"}
// Dependencies: {}
# [test] fn test_expr_with_attr () { check (r#"
macro_rules! m { ($a:expr) => { ok!(); } }
m!(#[allow(a)]());
"# , expect ! [[r#"
macro_rules! m { ($a:expr) => { ok!(); } }
ok!();
"#]] ,) }
};
}
