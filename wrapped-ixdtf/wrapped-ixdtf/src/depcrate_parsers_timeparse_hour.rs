// Generated macro for parse_hour (function)
macro_rules! Depcrate_parsers_timeparse_hour {
() => {
// Module: crate::parsers::time
// Provides: {"parse_hour"}
// Dependencies: {}
# [doc = " Parse an hour value."] # [inline] pub (crate) fn parse_hour < T : EncodingType > (cursor : & mut Cursor < T >) -> ParserResult < u8 > { let first = cursor . next_digit () ? . ok_or (ParseError :: TimeHour) ? ; let hour_value = first * 10 + cursor . next_digit () ? . ok_or (ParseError :: TimeHour) ? ; if ! (0 ..= 23) . contains (& hour_value) { return Err (ParseError :: TimeHour) ; } Ok (hour_value) }
};
}
