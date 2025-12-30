// Generated macro for test_date_from_weekday_of_month_opt (function)
macro_rules! Depcrate_naive_date_teststest_date_from_weekday_of_month_opt {
() => {
// Module: crate::naive::date::tests
// Provides: {"test_date_from_weekday_of_month_opt"}
// Dependencies: {}
# [test] fn test_date_from_weekday_of_month_opt () { let ymwd = NaiveDate :: from_weekday_of_month_opt ; assert_eq ! (ymwd (2018 , 8 , Weekday :: Tue , 0) , None) ; assert_eq ! (ymwd (2018 , 8 , Weekday :: Wed , 1) , Some (NaiveDate :: from_ymd_opt (2018 , 8 , 1) . unwrap ())) ; assert_eq ! (ymwd (2018 , 8 , Weekday :: Thu , 1) , Some (NaiveDate :: from_ymd_opt (2018 , 8 , 2) . unwrap ())) ; assert_eq ! (ymwd (2018 , 8 , Weekday :: Sun , 1) , Some (NaiveDate :: from_ymd_opt (2018 , 8 , 5) . unwrap ())) ; assert_eq ! (ymwd (2018 , 8 , Weekday :: Mon , 1) , Some (NaiveDate :: from_ymd_opt (2018 , 8 , 6) . unwrap ())) ; assert_eq ! (ymwd (2018 , 8 , Weekday :: Tue , 1) , Some (NaiveDate :: from_ymd_opt (2018 , 8 , 7) . unwrap ())) ; assert_eq ! (ymwd (2018 , 8 , Weekday :: Wed , 2) , Some (NaiveDate :: from_ymd_opt (2018 , 8 , 8) . unwrap ())) ; assert_eq ! (ymwd (2018 , 8 , Weekday :: Sun , 2) , Some (NaiveDate :: from_ymd_opt (2018 , 8 , 12) . unwrap ())) ; assert_eq ! (ymwd (2018 , 8 , Weekday :: Thu , 3) , Some (NaiveDate :: from_ymd_opt (2018 , 8 , 16) . unwrap ())) ; assert_eq ! (ymwd (2018 , 8 , Weekday :: Thu , 4) , Some (NaiveDate :: from_ymd_opt (2018 , 8 , 23) . unwrap ())) ; assert_eq ! (ymwd (2018 , 8 , Weekday :: Thu , 5) , Some (NaiveDate :: from_ymd_opt (2018 , 8 , 30) . unwrap ())) ; assert_eq ! (ymwd (2018 , 8 , Weekday :: Fri , 5) , Some (NaiveDate :: from_ymd_opt (2018 , 8 , 31) . unwrap ())) ; assert_eq ! (ymwd (2018 , 8 , Weekday :: Sat , 5) , None) ; }
};
}
