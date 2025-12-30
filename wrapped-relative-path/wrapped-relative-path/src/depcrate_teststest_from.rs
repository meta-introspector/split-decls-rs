// Generated macro for test_from (function)
macro_rules! Depcrate_teststest_from {
() => {
// Module: crate::tests
// Provides: {"test_from"}
// Dependencies: {}
# [test] fn test_from () { assert_eq ! (rp ("foo/bar") . to_owned () , RelativePathBuf :: from (String :: from ("foo/bar")) ,) ; assert_eq ! (RelativePathBuf :: from (rp ("foo/bar")) , RelativePathBuf :: from ("foo/bar") ,) ; assert_eq ! (rp ("foo/bar") . to_owned () , RelativePathBuf :: from ("foo/bar") ,) ; assert_eq ! (&* Box ::< RelativePath >:: from (rp ("foo/bar")) , rp ("foo/bar")) ; assert_eq ! (&* Box ::< RelativePath >:: from (RelativePathBuf :: from ("foo/bar")) , rp ("foo/bar")) ; assert_eq ! (&* Arc ::< RelativePath >:: from (rp ("foo/bar")) , rp ("foo/bar")) ; assert_eq ! (&* Arc ::< RelativePath >:: from (RelativePathBuf :: from ("foo/bar")) , rp ("foo/bar")) ; assert_eq ! (&* Rc ::< RelativePath >:: from (rp ("foo/bar")) , rp ("foo/bar")) ; assert_eq ! (&* Rc ::< RelativePath >:: from (RelativePathBuf :: from ("foo/bar")) , rp ("foo/bar")) ; }
};
}
