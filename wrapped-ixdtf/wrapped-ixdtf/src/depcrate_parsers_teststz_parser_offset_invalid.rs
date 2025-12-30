// Generated macro for tz_parser_offset_invalid (function)
macro_rules! Depcrate_parsers_teststz_parser_offset_invalid {
() => {
// Module: crate::parsers::tests
// Provides: {"tz_parser_offset_invalid"}
// Dependencies: {}
# [test] fn tz_parser_offset_invalid () { use super :: TimeZoneParser ; let invalid_offset = "+0" ; let err = TimeZoneParser :: from_str (invalid_offset) . parse_offset () . unwrap_err () ; assert_eq ! (err , ParseError :: AbruptEnd { location : "digit" }) ; let invalid_offset = "00:00" ; let err = TimeZoneParser :: from_str (invalid_offset) . parse_offset () . unwrap_err () ; assert_eq ! (err , ParseError :: OffsetNeedsSign) ; let invalid_offset = "+08:00[" ; let err = TimeZoneParser :: from_str (invalid_offset) . parse_offset () . unwrap_err () ; assert_eq ! (err , ParseError :: InvalidEnd) ; let invalid_offset = "+08:00:00[" ; let err = TimeZoneParser :: from_str (invalid_offset) . parse_offset () . unwrap_err () ; assert_eq ! (err , ParseError :: InvalidEnd) ; let invalid_offset = "+08:0000" ; let err = TimeZoneParser :: from_str (invalid_offset) . parse_offset () . unwrap_err () ; assert_eq ! (err , ParseError :: UtcTimeSeparator) ; let invalid_offset = "+0800:00" ; let err = TimeZoneParser :: from_str (invalid_offset) . parse_offset () . unwrap_err () ; assert_eq ! (err , ParseError :: UtcTimeSeparator) ; }
};
}
