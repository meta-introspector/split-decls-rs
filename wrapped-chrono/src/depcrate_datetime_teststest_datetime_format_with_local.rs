// Generated macro for test_datetime_format_with_local (function)
macro_rules! Depcrate_datetime_teststest_datetime_format_with_local {
() => {
// Module: crate::datetime::tests
// Provides: {"test_datetime_format_with_local"}
// Dependencies: {}
# [test] # [cfg (feature = "clock")] fn test_datetime_format_with_local () { let dt = Local :: now () . with_month (5) . unwrap () ; assert_eq ! (dt . format ("%Y") . to_string () , dt . with_timezone (& Utc) . format ("%Y") . to_string ()) ; }
};
}
