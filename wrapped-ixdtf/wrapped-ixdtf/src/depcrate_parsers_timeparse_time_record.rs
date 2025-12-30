// Generated macro for parse_time_record (function)
macro_rules! Depcrate_parsers_timeparse_time_record {
() => {
// Module: crate::parsers::time
// Provides: {"parse_time_record"}
// Dependencies: {}
# [doc = " Parse `TimeRecord`"] pub (crate) fn parse_time_record < T : EncodingType > (cursor : & mut Cursor < T > ,) -> ParserResult < TimeRecord > { let hour = parse_hour (cursor) ? ; if ! cursor . check_or (false , | ch | is_time_separator (ch) || ch . is_ascii_digit ()) ? { return Ok (TimeRecord { hour , minute : 0 , second : 0 , fraction : None , }) ; } let separator_present = cursor . check_or (false , is_time_separator) ? ; cursor . advance_if (separator_present) ; let minute = parse_minute_second (cursor , false) ? ; if ! cursor . check_or (false , | ch | is_time_separator (ch) || ch . is_ascii_digit ()) ? { return Ok (TimeRecord { hour , minute , second : 0 , fraction : None , }) ; } let second_separator = cursor . check_or (false , is_time_separator) ? ; assert_syntax ! (separator_present == second_separator , TimeSeparator) ; cursor . advance_if (second_separator) ; let second = parse_minute_second (cursor , true) ? ; let fraction = parse_fraction (cursor) ? ; Ok (TimeRecord { hour , minute , second , fraction , }) }
};
}
