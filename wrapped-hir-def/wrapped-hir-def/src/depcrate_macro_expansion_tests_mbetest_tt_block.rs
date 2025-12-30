// Generated macro for test_tt_block (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_tt_block {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_tt_block"}
// Dependencies: {}
# [test] fn test_tt_block () { check (r#"
macro_rules! m { ($tt:tt) => { fn foo() $tt } }
m! { { 1; } }
"# , expect ! [[r#"
macro_rules! m { ($tt:tt) => { fn foo() $tt } }
fn foo() {
    1;
}
"#]] ,) ; }
};
}
