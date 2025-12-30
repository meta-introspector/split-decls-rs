// Generated macro for test_time_from_hms_micro (function)
macro_rules! Depcrate_naive_time_teststest_time_from_hms_micro {
() => {
// Module: crate::naive::time::tests
// Provides: {"test_time_from_hms_micro"}
// Dependencies: {}
# [test] fn test_time_from_hms_micro () { assert_eq ! (NaiveTime :: from_hms_micro_opt (3 , 5 , 7 , 0) , Some (NaiveTime :: from_hms_nano_opt (3 , 5 , 7 , 0) . unwrap ())) ; assert_eq ! (NaiveTime :: from_hms_micro_opt (3 , 5 , 7 , 333) , Some (NaiveTime :: from_hms_nano_opt (3 , 5 , 7 , 333_000) . unwrap ())) ; assert_eq ! (NaiveTime :: from_hms_micro_opt (3 , 5 , 7 , 777_777) , Some (NaiveTime :: from_hms_nano_opt (3 , 5 , 7 , 777_777_000) . unwrap ())) ; assert_eq ! (NaiveTime :: from_hms_micro_opt (3 , 5 , 59 , 1_999_999) , Some (NaiveTime :: from_hms_nano_opt (3 , 5 , 59 , 1_999_999_000) . unwrap ())) ; assert_eq ! (NaiveTime :: from_hms_micro_opt (3 , 5 , 59 , 2_000_000) , None) ; assert_eq ! (NaiveTime :: from_hms_micro_opt (3 , 5 , 59 , 5_000_000) , None) ; assert_eq ! (NaiveTime :: from_hms_micro_opt (3 , 5 , 59 , u32 :: MAX) , None) ; }
};
}
