// Generated macro for test_to_string_round_trip_with_local (function)
macro_rules! Depcrate_datetime_teststest_to_string_round_trip_with_local {
() => {
// Module: crate::datetime::tests
// Provides: {"test_to_string_round_trip_with_local"}
// Dependencies: {}
# [test] # [cfg (feature = "clock")] fn test_to_string_round_trip_with_local () { let ndt = Local :: now () ; let _dt : DateTime < FixedOffset > = ndt . to_string () . parse () . unwrap () ; }
};
}
