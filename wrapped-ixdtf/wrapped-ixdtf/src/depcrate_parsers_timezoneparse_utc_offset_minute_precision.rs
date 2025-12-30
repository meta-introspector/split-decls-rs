// Generated macro for parse_utc_offset_minute_precision (function)
macro_rules! Depcrate_parsers_timezoneparse_utc_offset_minute_precision {
() => {
// Module: crate::parsers::timezone
// Provides: {"parse_utc_offset_minute_precision"}
// Dependencies: {}
# [doc = " Parse an `UtcOffsetMinutePrecision` node"] # [doc = ""] # [doc = " Returns the offset and whether the utc parsing includes a minute separator."] pub (crate) fn parse_utc_offset_minute_precision < T : EncodingType > (cursor : & mut Cursor < T > ,) -> ParserResult < (MinutePrecisionOffset , bool) > { let sign = cursor . next_or (ParseError :: abrupt_end ("time-numoffset")) ? ; if ! is_ascii_sign (sign) { return Err (ParseError :: OffsetNeedsSign) ; } let sign = Sign :: from (sign == b'+') ; let hour = parse_hour (cursor) ? ; if ! cursor . check_or (false , | ch | ch . is_ascii_digit () || is_time_separator (ch)) ? { let offset = MinutePrecisionOffset { sign , hour , minute : 0 , } ; return Ok ((offset , false)) ; } let separated = cursor . check_or (false , is_time_separator) ? ; cursor . advance_if (separated) ; let minute = parse_minute_second (cursor , false) ? ; Ok ((MinutePrecisionOffset { sign , hour , minute } , separated)) }
};
}
