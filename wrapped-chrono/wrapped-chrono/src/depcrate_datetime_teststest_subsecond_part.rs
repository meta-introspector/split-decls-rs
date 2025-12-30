// Generated macro for test_subsecond_part (function)
macro_rules! Depcrate_datetime_teststest_subsecond_part {
() => {
// Module: crate::datetime::tests
// Provides: {"test_subsecond_part"}
// Dependencies: {}
# [test] fn test_subsecond_part () { let datetime = Utc . from_local_datetime (& NaiveDate :: from_ymd_opt (2014 , 7 , 8) . unwrap () . and_hms_nano_opt (9 , 10 , 11 , 1234567) . unwrap () ,) . unwrap () ; assert_eq ! (1 , datetime . timestamp_subsec_millis ()) ; assert_eq ! (1234 , datetime . timestamp_subsec_micros ()) ; assert_eq ! (1234567 , datetime . timestamp_subsec_nanos ()) ; }
};
}
