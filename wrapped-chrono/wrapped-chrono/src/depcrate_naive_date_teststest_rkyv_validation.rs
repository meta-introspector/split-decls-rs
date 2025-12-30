// Generated macro for test_rkyv_validation (function)
macro_rules! Depcrate_naive_date_teststest_rkyv_validation {
() => {
// Module: crate::naive::date::tests
// Provides: {"test_rkyv_validation"}
// Dependencies: {}
# [test] # [cfg (feature = "rkyv-validation")] fn test_rkyv_validation () { let date_min = NaiveDate :: MIN ; let bytes = rkyv :: to_bytes :: < _ , 4 > (& date_min) . unwrap () ; assert_eq ! (rkyv :: from_bytes ::< NaiveDate > (& bytes) . unwrap () , date_min) ; let date_max = NaiveDate :: MAX ; let bytes = rkyv :: to_bytes :: < _ , 4 > (& date_max) . unwrap () ; assert_eq ! (rkyv :: from_bytes ::< NaiveDate > (& bytes) . unwrap () , date_max) ; }
};
}
