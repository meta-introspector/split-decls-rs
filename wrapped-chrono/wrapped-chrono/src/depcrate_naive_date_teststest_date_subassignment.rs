// Generated macro for test_date_subassignment (function)
macro_rules! Depcrate_naive_date_teststest_date_subassignment {
() => {
// Module: crate::naive::date::tests
// Provides: {"test_date_subassignment"}
// Dependencies: {}
# [test] fn test_date_subassignment () { let ymd = | y , m , d | NaiveDate :: from_ymd_opt (y , m , d) . unwrap () ; let mut date = ymd (2016 , 10 , 11) ; date -= TimeDelta :: try_days (10) . unwrap () ; assert_eq ! (date , ymd (2016 , 10 , 1)) ; date -= TimeDelta :: try_days (2) . unwrap () ; assert_eq ! (date , ymd (2016 , 9 , 29)) ; }
};
}
