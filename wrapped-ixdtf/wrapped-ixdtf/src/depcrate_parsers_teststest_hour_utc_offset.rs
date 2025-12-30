// Generated macro for test_hour_utc_offset (function)
macro_rules! Depcrate_parsers_teststest_hour_utc_offset {
() => {
// Module: crate::parsers::tests
// Provides: {"test_hour_utc_offset"}
// Dependencies: {}
# [test] fn test_hour_utc_offset () { let tz_test = "2024-08-24T14:00:00-05[-05]" ; let result = IxdtfParser :: from_str (tz_test) . parse () ; assert ! (result . is_ok ()) ; let tz_test = "2024-08-24T14:00:00-05" ; let result = IxdtfParser :: from_str (tz_test) . parse () ; assert ! (result . is_ok ()) ; }
};
}
