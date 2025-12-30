// Generated macro for test_into_os_string (function)
macro_rules! Depcrate_teststest_into_os_string {
() => {
// Module: crate::tests
// Provides: {"test_into_os_string"}
// Dependencies: {}
# [test] fn test_into_os_string () { let short = "short" ; let long = "i am a long string that will be allocated on the heap" ; let s = std :: ffi :: OsString :: from (CompactString :: new (short)) ; assert_eq ! (s . as_os_str () . to_str () . unwrap () , short) ; let l = std :: ffi :: OsString :: from (CompactString :: new (long)) ; assert_eq ! (l . as_os_str () . to_str () . unwrap () , long) ; }
};
}
