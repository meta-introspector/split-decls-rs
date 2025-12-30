// Generated macro for invalid_month (function)
macro_rules! Depcrate_parsers_testsinvalid_month {
() => {
// Module: crate::parsers::tests
// Provides: {"invalid_month"}
// Dependencies: {}
# [test] fn invalid_month () { let bad_value = "2021-00-29" ; let mut ixdtf = IxdtfParser :: from_str (bad_value) ; let err = ixdtf . parse () ; assert_eq ! (err , Err (ParseError :: InvalidMonthRange) , "Invalid month range parsing: \"{bad_value}\" should fail to parse.") ; let bad_value = "1900-13-29" ; let mut ixdtf = IxdtfParser :: from_str (bad_value) ; let err = ixdtf . parse () ; assert_eq ! (err , Err (ParseError :: InvalidMonthRange) , "Invalid month range parsing: \"{bad_value}\" should fail to parse.") ; }
};
}
