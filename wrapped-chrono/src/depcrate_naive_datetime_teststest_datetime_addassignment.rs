// Generated macro for test_datetime_addassignment (function)
macro_rules! Depcrate_naive_datetime_teststest_datetime_addassignment {
() => {
// Module: crate::naive::datetime::tests
// Provides: {"test_datetime_addassignment"}
// Dependencies: {}
# [test] fn test_datetime_addassignment () { let ymdhms = | y , m , d , h , n , s | NaiveDate :: from_ymd_opt (y , m , d) . unwrap () . and_hms_opt (h , n , s) . unwrap () ; let mut date = ymdhms (2016 , 10 , 1 , 10 , 10 , 10) ; date += TimeDelta :: try_minutes (10_000_000) . unwrap () ; assert_eq ! (date , ymdhms (2035 , 10 , 6 , 20 , 50 , 10)) ; date += TimeDelta :: try_days (10) . unwrap () ; assert_eq ! (date , ymdhms (2035 , 10 , 16 , 20 , 50 , 10)) ; }
};
}
