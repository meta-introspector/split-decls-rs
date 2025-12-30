// Generated macro for test_rfc3339_opts_nonexhaustive (function)
macro_rules! Depcrate_datetime_teststest_rfc3339_opts_nonexhaustive {
() => {
// Module: crate::datetime::tests
// Provides: {"test_rfc3339_opts_nonexhaustive"}
// Dependencies: {}
# [test] # [should_panic] # [cfg (feature = "alloc")] fn test_rfc3339_opts_nonexhaustive () { use crate :: SecondsFormat ; let dt = Utc . with_ymd_and_hms (1999 , 10 , 9 , 1 , 2 , 3) . unwrap () ; let _ = dt . to_rfc3339_opts (SecondsFormat :: __NonExhaustive , true) ; }
};
}
