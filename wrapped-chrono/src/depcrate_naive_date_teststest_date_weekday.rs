// Generated macro for test_date_weekday (function)
macro_rules! Depcrate_naive_date_teststest_date_weekday {
() => {
// Module: crate::naive::date::tests
// Provides: {"test_date_weekday"}
// Dependencies: {}
# [test] fn test_date_weekday () { assert_eq ! (NaiveDate :: from_ymd_opt (1582 , 10 , 15) . unwrap () . weekday () , Weekday :: Fri) ; assert_eq ! (NaiveDate :: from_ymd_opt (1875 , 5 , 20) . unwrap () . weekday () , Weekday :: Thu) ; assert_eq ! (NaiveDate :: from_ymd_opt (2000 , 1 , 1) . unwrap () . weekday () , Weekday :: Sat) ; }
};
}
