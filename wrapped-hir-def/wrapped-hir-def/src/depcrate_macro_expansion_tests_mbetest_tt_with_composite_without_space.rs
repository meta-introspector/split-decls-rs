// Generated macro for test_tt_with_composite_without_space (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_tt_with_composite_without_space {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_tt_with_composite_without_space"}
// Dependencies: {}
# [test] fn test_tt_with_composite_without_space () { check (r#"
macro_rules! m { ($ op:tt, $j:path) => ( ok!(); ) }
m!(==,Foo::Bool)
"# , expect ! [[r#"
macro_rules! m { ($ op:tt, $j:path) => ( ok!(); ) }
ok!();
"#]] ,) ; }
};
}
