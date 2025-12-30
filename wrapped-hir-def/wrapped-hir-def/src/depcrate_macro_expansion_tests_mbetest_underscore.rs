// Generated macro for test_underscore (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_underscore {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_underscore"}
// Dependencies: {}
# [test] fn test_underscore () { check (r#"
macro_rules! m { ($_:tt) => { ok!(); } }
m! { => }
"# , expect ! [[r#"
macro_rules! m { ($_:tt) => { ok!(); } }
ok!();
"#]] ,) ; }
};
}
