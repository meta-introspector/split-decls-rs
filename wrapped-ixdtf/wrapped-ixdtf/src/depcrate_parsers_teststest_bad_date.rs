// Generated macro for test_bad_date (function)
macro_rules! Depcrate_parsers_teststest_bad_date {
() => {
// Module: crate::parsers::tests
// Provides: {"test_bad_date"}
// Dependencies: {}
# [test] fn test_bad_date () { let dt = "-2022-06-05" ; let err = IxdtfParser :: from_str (dt) . parse () ; assert_eq ! (err , Err (ParseError :: DateExtendedYear)) ; let dt = "!2022-06-05" ; let err = IxdtfParser :: from_str (dt) . parse () ; assert_eq ! (err , Err (ParseError :: DateYear)) ; let dt = "20-06-05" ; let err = IxdtfParser :: from_str (dt) . parse () ; assert_eq ! (err , Err (ParseError :: DateYear)) ; let dt = "2022-0605" ; let err = IxdtfParser :: from_str (dt) . parse () ; assert_eq ! (err , Err (ParseError :: DateSeparator)) ; let dt = "202206-05" ; let err = IxdtfParser :: from_str (dt) . parse () ; assert_eq ! (err , Err (ParseError :: DateSeparator)) ; let dt = "2022-06-05e" ; let err = IxdtfParser :: from_str (dt) . parse () ; assert_eq ! (err , Err (ParseError :: InvalidEnd)) ; }
};
}
