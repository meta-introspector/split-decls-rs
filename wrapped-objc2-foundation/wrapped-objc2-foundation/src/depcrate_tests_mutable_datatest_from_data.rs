// Generated macro for test_from_data (function)
macro_rules! Depcrate_tests_mutable_datatest_from_data {
() => {
// Module: crate::tests::mutable_data
// Provides: {"test_from_data"}
// Dependencies: {}
# [test] fn test_from_data () { let data = NSData :: with_bytes (& [1 , 2]) ; let mut_data = NSMutableData :: dataWithData (& data) ; assert_eq ! (&* data , &** mut_data) ; }
};
}
