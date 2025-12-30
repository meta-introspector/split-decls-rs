// Generated macro for test_datetime_add_days (function)
macro_rules! Depcrate_datetime_teststest_datetime_add_days {
() => {
// Module: crate::datetime::tests
// Provides: {"test_datetime_add_days"}
// Dependencies: {}
# [test] fn test_datetime_add_days () { let est = FixedOffset :: west_opt (5 * 60 * 60) . unwrap () ; let kst = FixedOffset :: east_opt (9 * 60 * 60) . unwrap () ; assert_eq ! (format ! ("{}" , est . with_ymd_and_hms (2014 , 5 , 6 , 7 , 8 , 9) . unwrap () + Days :: new (5)) , "2014-05-11 07:08:09 -05:00") ; assert_eq ! (format ! ("{}" , kst . with_ymd_and_hms (2014 , 5 , 6 , 7 , 8 , 9) . unwrap () + Days :: new (5)) , "2014-05-11 07:08:09 +09:00") ; assert_eq ! (format ! ("{}" , est . with_ymd_and_hms (2014 , 5 , 6 , 7 , 8 , 9) . unwrap () + Days :: new (35)) , "2014-06-10 07:08:09 -05:00") ; assert_eq ! (format ! ("{}" , kst . with_ymd_and_hms (2014 , 5 , 6 , 7 , 8 , 9) . unwrap () + Days :: new (35)) , "2014-06-10 07:08:09 +09:00") ; assert_eq ! (format ! ("{}" , DstTester . with_ymd_and_hms (2014 , 4 , 6 , 7 , 8 , 9) . unwrap () + Days :: new (5)) , "2014-04-11 07:08:09 +09:00") ; assert_eq ! (format ! ("{}" , DstTester . with_ymd_and_hms (2014 , 4 , 6 , 7 , 8 , 9) . unwrap () + Days :: new (10)) , "2014-04-16 07:08:09 +08:00") ; assert_eq ! (format ! ("{}" , DstTester . with_ymd_and_hms (2014 , 9 , 6 , 7 , 8 , 9) . unwrap () + Days :: new (5)) , "2014-09-11 07:08:09 +08:00") ; assert_eq ! (format ! ("{}" , DstTester . with_ymd_and_hms (2014 , 9 , 6 , 7 , 8 , 9) . unwrap () + Days :: new (10)) , "2014-09-16 07:08:09 +09:00") ; }
};
}
