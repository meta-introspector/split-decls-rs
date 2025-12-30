// Generated macro for test_parse_macro_def_simple (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_parse_macro_def_simple {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_parse_macro_def_simple"}
// Dependencies: {}
# [test] fn test_parse_macro_def_simple () { cov_mark :: check ! (parse_macro_def_simple) ; check (r#"
macro m($id:ident) { fn $id() {} }
m!(bar);
"# , expect ! [[r#"
macro m($id:ident) { fn $id() {} }
fn bar() {}
"#]] ,) ; }
};
}
