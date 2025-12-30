// Generated macro for test_match_group_pattern_with_multiple_statement (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_match_group_pattern_with_multiple_statement {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_match_group_pattern_with_multiple_statement"}
// Dependencies: {}
# [test] fn test_match_group_pattern_with_multiple_statement () { check (r#"
macro_rules! m {
    ($($i:ident),*) => ( fn baz() { $($i ();)* } );
}
m! { foo, bar }
"# , expect ! [[r#"
macro_rules! m {
    ($($i:ident),*) => ( fn baz() { $($i ();)* } );
}
fn baz() {
    foo();
    bar();
}
"#]] ,) }
};
}
