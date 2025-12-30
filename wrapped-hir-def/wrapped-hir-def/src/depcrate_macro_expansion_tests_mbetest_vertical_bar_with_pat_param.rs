// Generated macro for test_vertical_bar_with_pat_param (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_vertical_bar_with_pat_param {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_vertical_bar_with_pat_param"}
// Dependencies: {}
# [test] fn test_vertical_bar_with_pat_param () { check (r#"
macro_rules! m { (|$pat:pat_param| ) => { ok!(); } }
m! { |x| }
 "# , expect ! [[r#"
macro_rules! m { (|$pat:pat_param| ) => { ok!(); } }
ok!();
 "#]] ,) ; }
};
}
