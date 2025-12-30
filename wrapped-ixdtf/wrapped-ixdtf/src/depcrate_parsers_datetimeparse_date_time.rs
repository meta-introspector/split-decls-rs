// Generated macro for parse_date_time (function)
macro_rules! Depcrate_parsers_datetimeparse_date_time {
() => {
// Module: crate::parsers::datetime
// Provides: {"parse_date_time"}
// Dependencies: {}
# [doc = " Parses a `DateTime` record."] fn parse_date_time < T : EncodingType > (cursor : & mut Cursor < T >) -> ParserResult < DateTimeRecord > { let date = parse_date (cursor) ? ; if ! cursor . check_or (false , is_date_time_separator) ? { return Ok (DateTimeRecord { date : Some (date) , time : None , time_zone : None , }) ; } cursor . advance () ; let time = parse_time_record (cursor) ? ; let time_zone = if cursor . check_or (false , | ch | is_ascii_sign (ch) || is_utc_designator (ch)) ? { Some (timezone :: parse_date_time_utc_offset (cursor) ?) } else { None } ; Ok (DateTimeRecord { date : Some (date) , time : Some (time) , time_zone , }) }
};
}
