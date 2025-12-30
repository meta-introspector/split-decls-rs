// Generated macro for test_path_with_path (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_path_with_path {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_path_with_path"}
// Dependencies: {}
# [test] fn test_path_with_path () { check (r#"
macro_rules! m {
    ($p:path) => { fn foo() { let a = $p::bar; } }
}
m! { foo }
"# , expect ! [[r#"
macro_rules! m {
    ($p:path) => { fn foo() { let a = $p::bar; } }
}
fn foo() {
    let a = foo::bar;
}
"#]] ,) ; }
};
}
