// Generated macro for test_relative_to (function)
macro_rules! Depcrate_teststest_relative_to {
() => {
// Module: crate::tests
// Provides: {"test_relative_to"}
// Dependencies: {}
# [test] fn test_relative_to () { assert_eq ! (rp ("foo/foo/bar") , rp ("foo/bar") . join_normalized ("../foo/bar")) ; assert_eq ! (rp ("../c/e") , rp ("x/y") . join_normalized ("../../a/b/../../../c/d/../e")) ; }
};
}
