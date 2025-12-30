// Generated macro for duplicate_same_calendar (function)
macro_rules! Depcrate_parsers_testsduplicate_same_calendar {
() => {
// Module: crate::parsers::tests
// Provides: {"duplicate_same_calendar"}
// Dependencies: {}
# [test] fn duplicate_same_calendar () { let duplicate_calendars = ["2020-11-11[!u-ca=iso8601][u-ca=iso8601]" , "2020-11-11[u-ca=iso8601][!u-ca=iso8601]" ,] ; for duplicate in duplicate_calendars { let result = IxdtfParser :: from_str (duplicate) . parse () . unwrap () ; let calendar = result . calendar . unwrap () ; assert_eq ! (calendar , "iso8601" . as_bytes () , "Invalid Ixdtf parsing: \"{duplicate}\" should fail parsing.") ; } }
};
}
