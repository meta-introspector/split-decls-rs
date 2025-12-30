// Generated macro for test_into_path_buf (function)
macro_rules! Depcrate_teststest_into_path_buf {
() => {
// Module: crate::tests
// Provides: {"test_into_path_buf"}
// Dependencies: {}
# [test] fn test_into_path_buf () { let short = "short" ; let long = "i am a long string that will be allocated on the heap" ; let s = std :: path :: PathBuf :: from (CompactString :: new (short)) ; assert_eq ! (s . as_os_str () . to_str () . unwrap () , short) ; let l = std :: path :: PathBuf :: from (CompactString :: new (long)) ; assert_eq ! (l . as_os_str () . to_str () . unwrap () , long) ; }
};
}
