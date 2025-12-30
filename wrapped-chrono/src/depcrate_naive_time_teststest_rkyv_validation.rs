// Generated macro for test_rkyv_validation (function)
macro_rules! Depcrate_naive_time_teststest_rkyv_validation {
() => {
// Module: crate::naive::time::tests
// Provides: {"test_rkyv_validation"}
// Dependencies: {}
# [test] # [cfg (feature = "rkyv-validation")] fn test_rkyv_validation () { let t_min = NaiveTime :: MIN ; let bytes = rkyv :: to_bytes :: < _ , 8 > (& t_min) . unwrap () ; assert_eq ! (rkyv :: from_bytes ::< NaiveTime > (& bytes) . unwrap () , t_min) ; let t_max = NaiveTime :: MAX ; let bytes = rkyv :: to_bytes :: < _ , 8 > (& t_max) . unwrap () ; assert_eq ! (rkyv :: from_bytes ::< NaiveTime > (& bytes) . unwrap () , t_max) ; }
};
}
