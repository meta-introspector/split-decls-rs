// Generated macro for test_parse_macro_def_rules (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_parse_macro_def_rules {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_parse_macro_def_rules"}
// Dependencies: {}
# [test] fn test_parse_macro_def_rules () { cov_mark :: check ! (parse_macro_def_rules) ; check (r#"
macro m {
    ($id:ident) => { fn $id() {} }
}
m!(bar);
"# , expect ! [[r#"
macro m {
    ($id:ident) => { fn $id() {} }
}
fn bar() {}
"#]] ,) ; }
};
}
