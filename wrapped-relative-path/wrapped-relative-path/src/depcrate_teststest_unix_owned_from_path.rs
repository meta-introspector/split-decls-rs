// Generated macro for test_unix_owned_from_path (function)
macro_rules! Depcrate_teststest_unix_owned_from_path {
() => {
// Module: crate::tests
// Provides: {"test_unix_owned_from_path"}
// Dependencies: {}
# [cfg (unix)] # [test] pub fn test_unix_owned_from_path () { use std :: ffi :: OsStr ; use std :: os :: unix :: ffi :: OsStrExt ; assert_eq ! (Err (FromPathErrorKind :: NonRelative . into ()) , RelativePathBuf :: from_path (Path :: new ("/foo/bar"))) ; let non_utf8 = OsStr :: from_bytes (& [0x80u8]) ; assert_eq ! (Err (FromPathErrorKind :: NonUtf8 . into ()) , RelativePathBuf :: from_path (Path :: new (non_utf8))) ; }
};
}
