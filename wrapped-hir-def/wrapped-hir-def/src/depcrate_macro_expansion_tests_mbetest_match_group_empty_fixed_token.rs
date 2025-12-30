// Generated macro for test_match_group_empty_fixed_token (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_match_group_empty_fixed_token {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_match_group_empty_fixed_token"}
// Dependencies: {}
# [test] fn test_match_group_empty_fixed_token () { check (r#"
macro_rules! m {
    ($($i:ident)* #abc) => ( fn baz() { $($i ();)* } );
}
m!{#abc}
"# , expect ! [[r#"
macro_rules! m {
    ($($i:ident)* #abc) => ( fn baz() { $($i ();)* } );
}
fn baz() {}
"#]] ,) }
};
}
