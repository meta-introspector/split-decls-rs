// Generated macro for test_date_addassignment (function)
macro_rules! Depcrate_naive_date_teststest_date_addassignment {
() => {
// Module: crate::naive::date::tests
// Provides: {"test_date_addassignment"}
// Dependencies: {}
# [test] fn test_date_addassignment () { let ymd = | y , m , d | NaiveDate :: from_ymd_opt (y , m , d) . unwrap () ; let mut date = ymd (2016 , 10 , 1) ; date += TimeDelta :: try_days (10) . unwrap () ; assert_eq ! (date , ymd (2016 , 10 , 11)) ; date += TimeDelta :: try_days (30) . unwrap () ; assert_eq ! (date , ymd (2016 , 11 , 10)) ; }
};
}
