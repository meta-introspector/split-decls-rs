// Generated macro for test_core_duration_ops (function)
macro_rules! Depcrate_datetime_teststest_core_duration_ops {
() => {
// Module: crate::datetime::tests
// Provides: {"test_core_duration_ops"}
// Dependencies: {}
# [test] fn test_core_duration_ops () { use core :: time :: Duration ; let mut utc_dt = Utc . with_ymd_and_hms (2023 , 8 , 29 , 11 , 34 , 12) . unwrap () ; let same = utc_dt + Duration :: ZERO ; assert_eq ! (utc_dt , same) ; utc_dt += Duration :: new (3600 , 0) ; assert_eq ! (utc_dt , Utc . with_ymd_and_hms (2023 , 8 , 29 , 12 , 34 , 12) . unwrap ()) ; }
};
}
