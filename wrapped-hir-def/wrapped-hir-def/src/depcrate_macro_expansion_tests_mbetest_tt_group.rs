// Generated macro for test_tt_group (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_tt_group {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_tt_group"}
// Dependencies: {}
# [test] fn test_tt_group () { check (r#"
macro_rules! m { ($($tt:tt)*) => { $($tt)* } }
m! { fn foo() {} }"
"# , expect ! [[r#"
macro_rules! m { ($($tt:tt)*) => { $($tt)* } }
fn foo() {}"
"#]] ,) ; }
};
}
