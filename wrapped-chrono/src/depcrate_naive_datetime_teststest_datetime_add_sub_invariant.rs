// Generated macro for test_datetime_add_sub_invariant (function)
macro_rules! Depcrate_naive_datetime_teststest_datetime_add_sub_invariant {
() => {
// Module: crate::naive::datetime::tests
// Provides: {"test_datetime_add_sub_invariant"}
// Dependencies: {}
# [test] fn test_datetime_add_sub_invariant () { let base = NaiveDate :: from_ymd_opt (2000 , 1 , 1) . unwrap () . and_hms_opt (0 , 0 , 0) . unwrap () ; let t = - 946684799990000 ; let time = base + TimeDelta :: microseconds (t) ; assert_eq ! (t , time . signed_duration_since (base) . num_microseconds () . unwrap ()) ; }
};
}
