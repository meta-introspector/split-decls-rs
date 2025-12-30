// Generated macro for test_display (function)
macro_rules! Depcrate_teststest_display {
() => {
// Module: crate::tests
// Provides: {"test_display"}
// Dependencies: {}
# [test] pub fn test_display () { assert_eq ! (RelativePathBuf :: from ("foo/bar") . to_string () , "foo/bar") ; assert_eq ! (RelativePath :: new ("foo/bar") . to_string () , "foo/bar") ; assert_eq ! (format ! ("{}" , RelativePathBuf :: from ("foo/bar")) , "foo/bar") ; assert_eq ! (format ! ("{}" , RelativePath :: new ("foo/bar")) , "foo/bar") ; }
};
}
