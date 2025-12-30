// Generated macro for parse_utc_offset (function)
macro_rules! Depcrate_parsers_timezoneparse_utc_offset {
() => {
// Module: crate::parsers::timezone
// Provides: {"parse_utc_offset"}
// Dependencies: {}
# [doc = " Parse a potentially full precision `UtcOffset`"] pub (crate) fn parse_utc_offset < T : EncodingType > (cursor : & mut Cursor < T > ,) -> ParserResult < UtcOffsetRecord > { let (minute_precision_offset , separated) = parse_utc_offset_minute_precision (cursor) ? ; if ! cursor . check_or (false , | ch | ch . is_ascii_digit () || is_time_separator (ch)) ? { return Ok (UtcOffsetRecord :: MinutePrecision (minute_precision_offset)) ; } if Some (separated) != cursor . check (is_time_separator) ? { return Err (ParseError :: UtcTimeSeparator) ; } cursor . advance_if (cursor . check_or (false , is_time_separator) ?) ; let second = parse_minute_second (cursor , false) ? ; let fraction = parse_fraction (cursor) ? ; Ok (UtcOffsetRecord :: FullPrecisionOffset (FullPrecisionOffset { minute_precision_offset , second , fraction , })) }
};
}
