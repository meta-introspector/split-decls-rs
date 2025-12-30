// Generated macro for invalid_time (function)
macro_rules! Depcrate_parsers_testsinvalid_time {
() => {
// Module: crate::parsers::tests
// Provides: {"invalid_time"}
// Dependencies: {}
# [test] fn invalid_time () { let bad_value = "20240801" ; let err = IxdtfParser :: from_str (bad_value) . parse_time () ; assert_eq ! (err , Err (ParseError :: InvalidEnd) , "Invalid time parsing: \"{bad_value}\" should fail to parse.") ; let bad_value = "24-12-08" ; let err = IxdtfParser :: from_str (bad_value) . parse_time () ; assert_eq ! (err , Err (ParseError :: TimeHour) , "Invalid time parsing: \"{bad_value}\" should fail to parse.") ; let bad_value = "T19-12-08" ; let err = IxdtfParser :: from_str (bad_value) . parse_time () ; assert_eq ! (err , Err (ParseError :: InvalidEnd) , "Invalid time parsing: \"{bad_value}\" should fail to parse.") ; let bad_value = "T19:12-089" ; let err = IxdtfParser :: from_str (bad_value) . parse_time () ; assert_eq ! (err , Err (ParseError :: AbruptEnd { location : "digit" }) , "Invalid time parsing: \"{bad_value}\" should fail to parse.") ; let bad_value = "T19:120-08" ; let err = IxdtfParser :: from_str (bad_value) . parse_time () ; assert_eq ! (err , Err (ParseError :: TimeSeparator) , "Invalid time parsing: \"{bad_value}\" should fail to parse.") ; }
};
}
