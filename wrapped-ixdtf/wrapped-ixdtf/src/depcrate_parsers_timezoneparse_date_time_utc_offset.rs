// Generated macro for parse_date_time_utc_offset (function)
macro_rules! Depcrate_parsers_timezoneparse_date_time_utc_offset {
() => {
// Module: crate::parsers::timezone
// Provides: {"parse_date_time_utc_offset"}
// Dependencies: {}
# [doc = " Parses a potentially full precision UTC offset or Z"] pub (crate) fn parse_date_time_utc_offset < T : EncodingType > (cursor : & mut Cursor < T > ,) -> ParserResult < UtcOffsetRecordOrZ > { if cursor . check_or (false , is_utc_designator) ? { cursor . advance () ; return Ok (UtcOffsetRecordOrZ :: Z) ; } let utc_offset = parse_utc_offset (cursor) ? ; Ok (UtcOffsetRecordOrZ :: Offset (utc_offset)) }
};
}
