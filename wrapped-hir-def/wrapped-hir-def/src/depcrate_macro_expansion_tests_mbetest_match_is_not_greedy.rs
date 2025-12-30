// Generated macro for test_match_is_not_greedy (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_match_is_not_greedy {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_match_is_not_greedy"}
// Dependencies: {}
# [test] fn test_match_is_not_greedy () { check (r#"
macro_rules! foo {
    ($($i:ident $(,)*),*) => {};
}
foo!(a,b);
"# , expect ! [[r#"
macro_rules! foo {
    ($($i:ident $(,)*),*) => {};
}

"#]] ,) ; }
};
}
