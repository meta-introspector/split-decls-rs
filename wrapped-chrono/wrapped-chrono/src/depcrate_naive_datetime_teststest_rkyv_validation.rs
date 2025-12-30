// Generated macro for test_rkyv_validation (function)
macro_rules! Depcrate_naive_datetime_teststest_rkyv_validation {
() => {
// Module: crate::naive::datetime::tests
// Provides: {"test_rkyv_validation"}
// Dependencies: {}
# [test] # [cfg (feature = "rkyv-validation")] fn test_rkyv_validation () { let dt_min = NaiveDateTime :: MIN ; let bytes = rkyv :: to_bytes :: < _ , 12 > (& dt_min) . unwrap () ; assert_eq ! (rkyv :: from_bytes ::< NaiveDateTime > (& bytes) . unwrap () , dt_min) ; let dt_max = NaiveDateTime :: MAX ; let bytes = rkyv :: to_bytes :: < _ , 12 > (& dt_max) . unwrap () ; assert_eq ! (rkyv :: from_bytes ::< NaiveDateTime > (& bytes) . unwrap () , dt_max) ; }
};
}
