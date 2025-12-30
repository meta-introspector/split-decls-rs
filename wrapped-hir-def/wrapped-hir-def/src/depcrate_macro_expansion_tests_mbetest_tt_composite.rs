// Generated macro for test_tt_composite (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_tt_composite {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_tt_composite"}
// Dependencies: {}
# [test] fn test_tt_composite () { check (r#"
macro_rules! m { ($tt:tt) => { ok!(); } }
m! { => }
m! { = > }
"# , expect ! [[r#"
macro_rules! m { ($tt:tt) => { ok!(); } }
ok!();
/* error: leftover tokens */ok!();
"#]] ,) ; }
};
}
