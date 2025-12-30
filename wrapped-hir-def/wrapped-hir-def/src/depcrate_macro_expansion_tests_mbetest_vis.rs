// Generated macro for test_vis (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_vis {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_vis"}
// Dependencies: {}
# [test] fn test_vis () { check (r#"
macro_rules! m {
    ($vis:vis $name:ident) => { $vis fn $name() {} }
}
m!(pub foo);
m!(foo);
"# , expect ! [[r#"
macro_rules! m {
    ($vis:vis $name:ident) => { $vis fn $name() {} }
}
pub fn foo() {}
fn foo() {}
"#]] ,) ; }
};
}
