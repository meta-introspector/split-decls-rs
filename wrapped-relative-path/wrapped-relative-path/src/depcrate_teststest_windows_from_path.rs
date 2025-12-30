// Generated macro for test_windows_from_path (function)
macro_rules! Depcrate_teststest_windows_from_path {
() => {
// Module: crate::tests
// Provides: {"test_windows_from_path"}
// Dependencies: {}
# [cfg (windows)] # [test] pub fn test_windows_from_path () { assert_eq ! (Err (FromPathErrorKind :: NonRelative . into ()) , RelativePath :: from_path ("c:\\foo\\bar")) ; assert_eq ! (Err (FromPathErrorKind :: BadSeparator . into ()) , RelativePath :: from_path ("foo\\bar")) ; }
};
}
