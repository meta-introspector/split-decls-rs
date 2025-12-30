// Generated macro for test_datetime_from_timestamp_millis (function)
macro_rules! Depcrate_datetime_teststest_datetime_from_timestamp_millis {
() => {
// Module: crate::datetime::tests
// Provides: {"test_datetime_from_timestamp_millis"}
// Dependencies: {}
# [test] fn test_datetime_from_timestamp_millis () { let valid_map = [(1662921288000 , "2022-09-11 18:34:48.000000000") , (1662921288123 , "2022-09-11 18:34:48.123000000") , (1662921287890 , "2022-09-11 18:34:47.890000000") , (- 2208936075000 , "1900-01-01 14:38:45.000000000") , (0 , "1970-01-01 00:00:00.000000000") , (119731017000 , "1973-10-17 18:36:57.000000000") , (1234567890000 , "2009-02-13 23:31:30.000000000") , (2034061609000 , "2034-06-16 09:06:49.000000000") ,] ; for (timestamp_millis , _formatted) in valid_map . iter () . copied () { let datetime = DateTime :: from_timestamp_millis (timestamp_millis) . unwrap () ; assert_eq ! (timestamp_millis , datetime . timestamp_millis ()) ; # [cfg (feature = "alloc")] assert_eq ! (datetime . format ("%F %T%.9f") . to_string () , _formatted) ; } let invalid = [i64 :: MAX , i64 :: MIN] ; for timestamp_millis in invalid . iter () . copied () { let datetime = DateTime :: from_timestamp_millis (timestamp_millis) ; assert ! (datetime . is_none ()) ; } let secs_test = [0 , 1 , 2 , 1000 , 1234 , 12345678 , - 1 , - 2 , - 1000 , - 12345678] ; for secs in secs_test . iter () . cloned () { assert_eq ! (DateTime :: from_timestamp_millis (secs * 1000) , DateTime :: from_timestamp_secs (secs)) ; } }
};
}
