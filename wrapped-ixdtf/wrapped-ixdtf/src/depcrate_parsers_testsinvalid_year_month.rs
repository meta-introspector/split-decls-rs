// Generated macro for invalid_year_month (function)
macro_rules! Depcrate_parsers_testsinvalid_year_month {
() => {
// Module: crate::parsers::tests
// Provides: {"invalid_year_month"}
// Dependencies: {}
# [test] fn invalid_year_month () { let bad_value = "+002020-11T12:28:32[!u-ca=iso8601]" ; let err = IxdtfParser :: from_str (bad_value) . parse_year_month () ; assert_eq ! (err , Err (ParseError :: InvalidEnd)) ; let bad_value = "-202011[!u-ca=iso8601]" ; let err = IxdtfParser :: from_str (bad_value) . parse_year_month () ; assert_eq ! (err , Err (ParseError :: DateMonth)) ; let bad_value = "-00202011Z[Europe/Berlin]" ; let err = IxdtfParser :: from_str (bad_value) . parse_year_month () ; assert_eq ! (err , Err (ParseError :: InvalidEnd)) ; }
};
}
