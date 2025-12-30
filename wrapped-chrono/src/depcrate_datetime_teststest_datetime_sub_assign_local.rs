// Generated macro for test_datetime_sub_assign_local (function)
macro_rules! Depcrate_datetime_teststest_datetime_sub_assign_local {
() => {
// Module: crate::datetime::tests
// Provides: {"test_datetime_sub_assign_local"}
// Dependencies: {}
# [test] # [cfg (feature = "clock")] fn test_datetime_sub_assign_local () { let naivedatetime = NaiveDate :: from_ymd_opt (2022 , 1 , 1) . unwrap () . and_hms_opt (0 , 0 , 0) . unwrap () ; let datetime = Local . from_utc_datetime (& naivedatetime) ; let mut datetime_sub = Local . from_utc_datetime (& naivedatetime) ; for i in 1 ..= 365 { datetime_sub -= TimeDelta :: try_days (1) . unwrap () ; assert_eq ! (datetime_sub , datetime - TimeDelta :: try_days (i) . unwrap ()) } }
};
}
