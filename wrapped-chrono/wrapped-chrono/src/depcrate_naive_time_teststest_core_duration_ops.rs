// Generated macro for test_core_duration_ops (function)
macro_rules! Depcrate_naive_time_teststest_core_duration_ops {
() => {
// Module: crate::naive::time::tests
// Provides: {"test_core_duration_ops"}
// Dependencies: {}
# [test] fn test_core_duration_ops () { use core :: time :: Duration ; let mut t = NaiveTime :: from_hms_opt (11 , 34 , 23) . unwrap () ; let same = t + Duration :: ZERO ; assert_eq ! (t , same) ; t += Duration :: new (3600 , 0) ; assert_eq ! (t , NaiveTime :: from_hms_opt (12 , 34 , 23) . unwrap ()) ; t -= Duration :: new (7200 , 0) ; assert_eq ! (t , NaiveTime :: from_hms_opt (10 , 34 , 23) . unwrap ()) ; }
};
}
