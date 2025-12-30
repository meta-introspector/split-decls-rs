// Generated macro for parse_utc_offset_minute_precision_strict (function)
macro_rules! Depcrate_parsers_timezoneparse_utc_offset_minute_precision_strict {
() => {
// Module: crate::parsers::timezone
// Provides: {"parse_utc_offset_minute_precision_strict"}
// Dependencies: {}
pub (crate) fn parse_utc_offset_minute_precision_strict < T : EncodingType > (cursor : & mut Cursor < T > ,) -> ParserResult < MinutePrecisionOffset > { let (offset , _) = parse_utc_offset_minute_precision (cursor) ? ; if cursor . check_or (false , | ch | is_time_separator (ch) || ch . is_ascii_digit ()) ? { return Err (ParseError :: InvalidMinutePrecisionOffset) ; } Ok (offset) }
};
}
