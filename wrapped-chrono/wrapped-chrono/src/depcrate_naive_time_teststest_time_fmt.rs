// Generated macro for test_time_fmt (function)
macro_rules! Depcrate_naive_time_teststest_time_fmt {
() => {
// Module: crate::naive::time::tests
// Provides: {"test_time_fmt"}
// Dependencies: {}
# [test] fn test_time_fmt () { assert_eq ! (format ! ("{}" , NaiveTime :: from_hms_milli_opt (23 , 59 , 59 , 999) . unwrap ()) , "23:59:59.999") ; assert_eq ! (format ! ("{}" , NaiveTime :: from_hms_milli_opt (23 , 59 , 59 , 1_000) . unwrap ()) , "23:59:60") ; assert_eq ! (format ! ("{}" , NaiveTime :: from_hms_milli_opt (23 , 59 , 59 , 1_001) . unwrap ()) , "23:59:60.001") ; assert_eq ! (format ! ("{}" , NaiveTime :: from_hms_micro_opt (0 , 0 , 0 , 43210) . unwrap ()) , "00:00:00.043210") ; assert_eq ! (format ! ("{}" , NaiveTime :: from_hms_nano_opt (0 , 0 , 0 , 6543210) . unwrap ()) , "00:00:00.006543210") ; assert_eq ! (format ! ("{:30}" , NaiveTime :: from_hms_milli_opt (3 , 5 , 7 , 9) . unwrap ()) , "03:05:07.009") ; }
};
}
