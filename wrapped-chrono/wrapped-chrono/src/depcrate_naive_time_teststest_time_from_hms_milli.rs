// Generated macro for test_time_from_hms_milli (function)
macro_rules! Depcrate_naive_time_teststest_time_from_hms_milli {
() => {
// Module: crate::naive::time::tests
// Provides: {"test_time_from_hms_milli"}
// Dependencies: {}
# [test] fn test_time_from_hms_milli () { assert_eq ! (NaiveTime :: from_hms_milli_opt (3 , 5 , 7 , 0) , Some (NaiveTime :: from_hms_nano_opt (3 , 5 , 7 , 0) . unwrap ())) ; assert_eq ! (NaiveTime :: from_hms_milli_opt (3 , 5 , 7 , 777) , Some (NaiveTime :: from_hms_nano_opt (3 , 5 , 7 , 777_000_000) . unwrap ())) ; assert_eq ! (NaiveTime :: from_hms_milli_opt (3 , 5 , 59 , 1_999) , Some (NaiveTime :: from_hms_nano_opt (3 , 5 , 59 , 1_999_000_000) . unwrap ())) ; assert_eq ! (NaiveTime :: from_hms_milli_opt (3 , 5 , 59 , 2_000) , None) ; assert_eq ! (NaiveTime :: from_hms_milli_opt (3 , 5 , 59 , 5_000) , None) ; assert_eq ! (NaiveTime :: from_hms_milli_opt (3 , 5 , 59 , u32 :: MAX) , None) ; }
};
}
