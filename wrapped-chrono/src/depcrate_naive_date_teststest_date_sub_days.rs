// Generated macro for test_date_sub_days (function)
macro_rules! Depcrate_naive_date_teststest_date_sub_days {
() => {
// Module: crate::naive::date::tests
// Provides: {"test_date_sub_days"}
// Dependencies: {}
# [test] fn test_date_sub_days () { fn check (lhs : Option < NaiveDate > , days : Days , rhs : Option < NaiveDate >) { assert_eq ! (lhs . unwrap () . checked_sub_days (days) , rhs) ; } let ymd = NaiveDate :: from_ymd_opt ; check (ymd (2014 , 1 , 1) , Days :: new (0) , ymd (2014 , 1 , 1)) ; check (ymd (2014 , 1 , 2) , Days :: new (1) , ymd (2014 , 1 , 1)) ; check (ymd (2014 , 12 , 31) , Days :: new (364) , ymd (2014 , 1 , 1)) ; check (ymd (2015 , 1 , 3) , Days :: new (365 + 2) , ymd (2014 , 1 , 1)) ; check (ymd (2018 , 1 , 1) , Days :: new (365 * 4 + 1) , ymd (2014 , 1 , 1)) ; check (ymd (2414 , 1 , 1) , Days :: new (365 * 400 + 97) , ymd (2014 , 1 , 1)) ; check (ymd (MAX_YEAR , 12 , 31) , Days :: new (MAX_DAYS_FROM_YEAR_0 . try_into () . unwrap ()) , ymd (0 , 1 , 1)) ; check (ymd (0 , 1 , 1) , Days :: new ((- MIN_DAYS_FROM_YEAR_0) . try_into () . unwrap ()) , ymd (MIN_YEAR , 1 , 1) ,) ; }
};
}
