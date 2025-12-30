// Generated macro for test_extract_lldb_version (function)
macro_rules! Depcrate_teststest_extract_lldb_version {
() => {
// Module: crate::tests
// Provides: {"test_extract_lldb_version"}
// Dependencies: {}
# [test] fn test_extract_lldb_version () { assert_eq ! (extract_lldb_version ("LLDB-179.5") , Some (179)) ; assert_eq ! (extract_lldb_version ("lldb-300.2.51") , Some (300)) ; assert_eq ! (extract_lldb_version ("lldb version 6.0.1") , Some (600)) ; assert_eq ! (extract_lldb_version ("lldb version 9.0.0") , Some (900)) ; }
};
}
