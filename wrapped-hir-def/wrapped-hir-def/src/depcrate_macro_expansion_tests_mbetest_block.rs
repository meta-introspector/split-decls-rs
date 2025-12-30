// Generated macro for test_block (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_block {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_block"}
// Dependencies: {}
# [test] fn test_block () { check (r#"
macro_rules! m { ($b:block) => { fn foo() $b } }
m! { { 1; } }
"# , expect ! [[r#"
macro_rules! m { ($b:block) => { fn foo() $b } }
fn foo() {
    1;
}
"#]] ,) ; }
};
}
