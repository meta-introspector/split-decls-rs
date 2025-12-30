// Generated macro for test_datetime_from_timestamp_secs (function)
macro_rules! Depcrate_datetime_teststest_datetime_from_timestamp_secs {
() => {
// Module: crate::datetime::tests
// Provides: {"test_datetime_from_timestamp_secs"}
// Dependencies: {}
# [test] fn test_datetime_from_timestamp_secs () { let valid = [- 2208936075 , 0 , 119731017 , 1234567890 , 2034061609] ; for timestamp_secs in valid . iter () . copied () { let datetime = DateTime :: from_timestamp_secs (timestamp_secs) . unwrap () ; assert_eq ! (timestamp_secs , datetime . timestamp ()) ; assert_eq ! (DateTime :: from_timestamp (timestamp_secs , 0) . unwrap () , datetime) ; } }
};
}
