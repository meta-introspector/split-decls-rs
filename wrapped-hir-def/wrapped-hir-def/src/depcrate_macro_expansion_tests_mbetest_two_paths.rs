// Generated macro for test_two_paths (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_two_paths {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_two_paths"}
// Dependencies: {}
# [test] fn test_two_paths () { check (r#"
macro_rules! m {
    ($i:path, $j:path) => { fn foo() { let a = $ i; let b = $j; } }
}
m! { foo, bar }
"# , expect ! [[r#"
macro_rules! m {
    ($i:path, $j:path) => { fn foo() { let a = $ i; let b = $j; } }
}
fn foo() {
    let a = foo;
    let b = bar;
}
"#]] ,) ; }
};
}
