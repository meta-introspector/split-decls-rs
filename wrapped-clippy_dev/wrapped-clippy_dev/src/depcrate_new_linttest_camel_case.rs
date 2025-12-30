// Generated macro for test_camel_case (function)
macro_rules! Depcrate_new_linttest_camel_case {
() => {
// Module: crate::new_lint
// Provides: {"test_camel_case"}
// Dependencies: {}
# [test] fn test_camel_case () { let s = "a_lint" ; let s2 = to_camel_case (s) ; assert_eq ! (s2 , "ALint") ; let name = "a_really_long_new_lint" ; let name2 = to_camel_case (name) ; assert_eq ! (name2 , "AReallyLongNewLint") ; let name3 = "lint__name" ; let name4 = to_camel_case (name3) ; assert_eq ! (name4 , "LintName") ; }
};
}
