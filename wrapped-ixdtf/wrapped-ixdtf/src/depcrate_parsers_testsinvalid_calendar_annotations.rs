// Generated macro for invalid_calendar_annotations (function)
macro_rules! Depcrate_parsers_testsinvalid_calendar_annotations {
() => {
// Module: crate::parsers::tests
// Provides: {"invalid_calendar_annotations"}
// Dependencies: {}
# [test] fn invalid_calendar_annotations () { let bad_value = "2021-01-29 02:12:48+01:00:00[!u-ca=iso8601][u-ca=japanese]" ; let err = IxdtfParser :: from_str (bad_value) . parse () ; assert_eq ! (err , Err (ParseError :: CriticalDuplicateCalendar) , "Invalid annotation parsing: \"{bad_value}\" should fail to parse.") ; let bad_value = "2021-01-29 02:12:48+01:00:00[u-ca=japanese][u-ca=iso8601][!u-ca=gregorian]" ; let err = IxdtfParser :: from_str (bad_value) . parse () ; assert_eq ! (err , Err (ParseError :: CriticalDuplicateCalendar) , "Invalid annotation parsing: \"{bad_value}\" should fail to parse.") ; }
};
}
