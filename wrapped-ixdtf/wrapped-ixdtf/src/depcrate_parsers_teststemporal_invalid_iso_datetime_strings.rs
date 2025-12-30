// Generated macro for temporal_invalid_iso_datetime_strings (function)
macro_rules! Depcrate_parsers_teststemporal_invalid_iso_datetime_strings {
() => {
// Module: crate::parsers::tests
// Provides: {"temporal_invalid_iso_datetime_strings"}
// Dependencies: {}
# [test] fn temporal_invalid_iso_datetime_strings () { const INVALID_DATETIME_STRINGS : [& str ; 32] = ["" , "invalid iso8601" , "2020-01-00" , "2020-01-32" , "2020-02-30" , "2021-02-29" , "2020-00-01" , "2020-13-01" , "2020-01-01T" , "2020-01-01T25:00:00" , "2020-01-01T01:60:00" , "2020-01-01T01:60:61" , "2020-01-01junk" , "2020-01-01T00:00:00junk" , "2020-01-01T00:00:00+00:00junk" , "2020-01-01T00:00:00+00:00[UTC]junk" , "2020-01-01T00:00:00+00:00[UTC][u-ca=iso8601]junk" , "02020-01-01" , "2020-001-01" , "2020-01-001" , "2020-01-01T001" , "2020-01-01T01:001" , "2020-01-01T01:01:001" , "2020-W01-1" , "2020-001" , "+0002020-01-01" , "2020-01" , "+002020-01" , "01-01" , "2020-W01" , "P1Y" , "-P12Y" ,] ; for invalid_target in INVALID_DATETIME_STRINGS { let error_result = IxdtfParser :: from_str (invalid_target) . parse () ; assert ! (error_result . is_err () , "Invalid ISO8601 `DateTime` target: \"{invalid_target}\" should fail parsing.") ; } }
};
}
