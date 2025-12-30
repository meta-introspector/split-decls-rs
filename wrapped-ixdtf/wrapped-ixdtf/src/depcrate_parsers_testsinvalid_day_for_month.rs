// Generated macro for invalid_day_for_month (function)
macro_rules! Depcrate_parsers_testsinvalid_day_for_month {
() => {
// Module: crate::parsers::tests
// Provides: {"invalid_day_for_month"}
// Dependencies: {}
# [test] fn invalid_day_for_month () { let bad_value = "2021-02-29" ; let err = IxdtfParser :: from_str (bad_value) . parse () ; assert_eq ! (err , Err (ParseError :: InvalidDayRange) , "Invalid day range parsing: \"{bad_value}\" should fail to parse.") ; let bad_value = "1900-02-29" ; let err = IxdtfParser :: from_str (bad_value) . parse () ; assert_eq ! (err , Err (ParseError :: InvalidDayRange) , "Invalid day range parsing: \"{bad_value}\" should fail to parse.") ; let bad_value = "2021-04-31" ; let err = IxdtfParser :: from_str (bad_value) . parse () ; assert_eq ! (err , Err (ParseError :: InvalidDayRange) , "Invalid day range parsing: \"{bad_value}\" should fail to parse.") ; let bad_value = "2021-04-00" ; let err = IxdtfParser :: from_str (bad_value) . parse () ; assert_eq ! (err , Err (ParseError :: InvalidDayRange) , "Invalid day range parsing: \"{bad_value}\" should fail to parse.") ; }
};
}
