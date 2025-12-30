// Generated macro for test_match_group_zero_match (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_match_group_zero_match {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_match_group_zero_match"}
// Dependencies: {}
# [test] fn test_match_group_zero_match () { check (r#"
macro_rules! m { ( $($i:ident)* ) => (); }
m!();
"# , expect ! [[r#"
macro_rules! m { ( $($i:ident)* ) => (); }

"#]] ,) ; }
};
}
