// Generated macro for test_core_duration_ops (function)
macro_rules! Depcrate_naive_datetime_teststest_core_duration_ops {
() => {
// Module: crate::naive::datetime::tests
// Provides: {"test_core_duration_ops"}
// Dependencies: {}
# [test] fn test_core_duration_ops () { use core :: time :: Duration ; let mut dt = NaiveDate :: from_ymd_opt (2023 , 8 , 29) . unwrap () . and_hms_opt (11 , 34 , 12) . unwrap () ; let same = dt + Duration :: ZERO ; assert_eq ! (dt , same) ; dt += Duration :: new (3600 , 0) ; assert_eq ! (dt , NaiveDate :: from_ymd_opt (2023 , 8 , 29) . unwrap () . and_hms_opt (12 , 34 , 12) . unwrap ()) ; }
};
}
