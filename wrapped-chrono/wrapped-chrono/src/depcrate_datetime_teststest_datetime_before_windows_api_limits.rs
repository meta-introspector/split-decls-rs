// Generated macro for test_datetime_before_windows_api_limits (function)
macro_rules! Depcrate_datetime_teststest_datetime_before_windows_api_limits {
() => {
// Module: crate::datetime::tests
// Provides: {"test_datetime_before_windows_api_limits"}
// Dependencies: {}
# [test] # [cfg (feature = "clock")] fn test_datetime_before_windows_api_limits () { let dt = NaiveDate :: from_ymd_opt (1601 , 1 , 1) . unwrap () . and_hms_milli_opt (4 , 5 , 22 , 122) . unwrap () ; let local_dt = Local . from_utc_datetime (& dt) ; dbg ! (local_dt) ; }
};
}
