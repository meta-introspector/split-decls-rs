// Generated macro for parse_date_day (function)
macro_rules! Depcrate_parsers_datetimeparse_date_day {
() => {
// Module: crate::parsers::datetime
// Provides: {"parse_date_day"}
// Dependencies: {}
# [inline] fn parse_date_day < T : EncodingType > (cursor : & mut Cursor < T >) -> ParserResult < u8 > { let first = cursor . next_digit () ? . ok_or (ParseError :: DateDay) ? ; let day_value = first * 10 + cursor . next_digit () ? . ok_or (ParseError :: DateDay) ? ; Ok (day_value) }
};
}
