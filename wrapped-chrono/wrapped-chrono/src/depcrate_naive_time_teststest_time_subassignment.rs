// Generated macro for test_time_subassignment (function)
macro_rules! Depcrate_naive_time_teststest_time_subassignment {
() => {
// Module: crate::naive::time::tests
// Provides: {"test_time_subassignment"}
// Dependencies: {}
# [test] fn test_time_subassignment () { let hms = | h , m , s | NaiveTime :: from_hms_opt (h , m , s) . unwrap () ; let mut time = hms (12 , 12 , 12) ; time -= TimeDelta :: try_hours (10) . unwrap () ; assert_eq ! (time , hms (2 , 12 , 12)) ; time -= TimeDelta :: try_hours (10) . unwrap () ; assert_eq ! (time , hms (16 , 12 , 12)) ; }
};
}
