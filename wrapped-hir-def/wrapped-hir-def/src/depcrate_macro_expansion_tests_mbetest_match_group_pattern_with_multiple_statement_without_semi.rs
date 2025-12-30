// Generated macro for test_match_group_pattern_with_multiple_statement_without_semi (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_match_group_pattern_with_multiple_statement_without_semi {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_match_group_pattern_with_multiple_statement_without_semi"}
// Dependencies: {}
# [test] fn test_match_group_pattern_with_multiple_statement_without_semi () { check (r#"
macro_rules! m {
    ($($i:ident),*) => ( fn baz() { $($i() );* } );
}
m! { foo, bar }
"# , expect ! [[r#"
macro_rules! m {
    ($($i:ident),*) => ( fn baz() { $($i() );* } );
}
fn baz() {
    foo();
    bar()
}
"#]] ,) }
};
}
