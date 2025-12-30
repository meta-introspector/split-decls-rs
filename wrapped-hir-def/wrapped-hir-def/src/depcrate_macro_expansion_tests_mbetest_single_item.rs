// Generated macro for test_single_item (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_single_item {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_single_item"}
// Dependencies: {}
# [test] fn test_single_item () { check (r#"
macro_rules! m { ($i:item) => ( $i ) }
m! { mod c {} }
"# , expect ! [[r#"
macro_rules! m { ($i:item) => ( $i ) }
mod c {}
"#]] ,) }
};
}
