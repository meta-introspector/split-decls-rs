// Generated macro for test_match_group_in_subtree (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_match_group_in_subtree {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_match_group_in_subtree"}
// Dependencies: {}
# [test] fn test_match_group_in_subtree () { check (r#"
macro_rules! m {
    (fn $name:ident { $($i:ident)* } ) => ( fn $name() { $($i ();)* } );
}
m! { fn baz { a b } }
"# , expect ! [[r#"
macro_rules! m {
    (fn $name:ident { $($i:ident)* } ) => ( fn $name() { $($i ();)* } );
}
fn baz() {
    a();
    b();
}
"#]] ,) }
};
}
