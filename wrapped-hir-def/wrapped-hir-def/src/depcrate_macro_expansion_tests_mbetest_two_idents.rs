// Generated macro for test_two_idents (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_two_idents {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_two_idents"}
// Dependencies: {}
# [test] fn test_two_idents () { check (r#"
macro_rules! m {
    ($i:ident, $j:ident) => { fn foo() { let a = $i; let b = $j; } }
}
m! { foo, bar }
"# , expect ! [[r#"
macro_rules! m {
    ($i:ident, $j:ident) => { fn foo() { let a = $i; let b = $j; } }
}
fn foo() {
    let a = foo;
    let b = bar;
}
"#]] ,) ; }
};
}
