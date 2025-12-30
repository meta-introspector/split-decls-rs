// Generated macro for test_bytes (function)
macro_rules! Depcrate_tests_datatest_bytes {
() => {
// Module: crate::tests::data
// Provides: {"test_bytes"}
// Dependencies: {}
# [test] fn test_bytes () { let bytes = [3 , 7 , 16 , 52 , 112 , 19] ; let data = NSData :: with_bytes (& bytes) ; assert_eq ! (data . len () , bytes . len ()) ; assert_eq ! (data . to_vec () , bytes) ; }
};
}
