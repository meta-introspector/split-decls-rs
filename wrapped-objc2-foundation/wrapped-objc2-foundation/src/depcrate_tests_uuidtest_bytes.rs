// Generated macro for test_bytes (function)
macro_rules! Depcrate_tests_uuidtest_bytes {
() => {
// Module: crate::tests::uuid
// Provides: {"test_bytes"}
// Dependencies: {}
# [test] # [ignore = "encoding depends on Foundation version"] fn test_bytes () { let uuid = NSUUID :: from_bytes ([10 ; 16]) ; assert_eq ! (uuid . as_bytes () , [10 ; 16]) ; }
};
}
