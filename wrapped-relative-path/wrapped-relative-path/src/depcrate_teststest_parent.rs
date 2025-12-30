// Generated macro for test_parent (function)
macro_rules! Depcrate_teststest_parent {
() => {
// Module: crate::tests
// Provides: {"test_parent"}
// Dependencies: {}
# [test] fn test_parent () { let path = rp ("baz/./bar/foo//./.") ; assert_eq ! (Some (rp ("baz/./bar")) , path . parent ()) ; assert_eq ! (Some (rp ("baz/.")) , path . parent () . and_then (RelativePath :: parent)) ; assert_eq ! (Some (rp ("")) , path . parent () . and_then (RelativePath :: parent) . and_then (RelativePath :: parent)) ; assert_eq ! (None , path . parent () . and_then (RelativePath :: parent) . and_then (RelativePath :: parent) . and_then (RelativePath :: parent)) ; }
};
}
