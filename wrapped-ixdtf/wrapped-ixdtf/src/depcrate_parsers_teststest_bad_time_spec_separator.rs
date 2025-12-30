// Generated macro for test_bad_time_spec_separator (function)
macro_rules! Depcrate_parsers_teststest_bad_time_spec_separator {
() => {
// Module: crate::parsers::tests
// Provides: {"test_bad_time_spec_separator"}
// Dependencies: {}
# [test] fn test_bad_time_spec_separator () { let dt = "2022-06-05  043422.000" ; let err = IxdtfParser :: from_str (dt) . parse () ; assert_eq ! (err , Err (ParseError :: TimeHour)) ; let dt = "2022-06-05 04:3422.000" ; let err = IxdtfParser :: from_str (dt) . parse () ; assert_eq ! (err , Err (ParseError :: TimeSeparator)) ; let dt = "2022-06-05 0434:22.000" ; let err = IxdtfParser :: from_str (dt) . parse () ; assert_eq ! (err , Err (ParseError :: TimeSeparator)) ; let dt = "2022-06-05 03422.000" ; let err = IxdtfParser :: from_str (dt) . parse () ; assert_eq ! (err , Err (ParseError :: TimeSecond)) ; let dt = "2022-06-05 3:42:22.000" ; let err = IxdtfParser :: from_str (dt) . parse () ; assert_eq ! (err , Err (ParseError :: TimeHour)) ; let dt = "2022-06-05 03:42:22;000" ; let err = IxdtfParser :: from_str (dt) . parse () ; assert_eq ! (err , Err (ParseError :: InvalidEnd)) ; }
};
}
