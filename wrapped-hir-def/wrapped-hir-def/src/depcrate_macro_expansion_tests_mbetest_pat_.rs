// Generated macro for test_pat_ (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_pat_ {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_pat_"}
// Dependencies: {}
# [test] fn test_pat_ () { check (r#"
macro_rules! m {
    ($p:pat) => { fn foo() { let $p; } }
}
m! { (a, b) }
"# , expect ! [[r#"
macro_rules! m {
    ($p:pat) => { fn foo() { let $p; } }
}
fn foo() {
    let (a, b);
}
"#]] ,) ; }
};
}
