// Generated macro for parse_minute_second (function)
macro_rules! Depcrate_parsers_timeparse_minute_second {
() => {
// Module: crate::parsers::time
// Provides: {"parse_minute_second"}
// Dependencies: {}
# [doc = " Parses `MinuteSecond` value."] # [inline] pub (crate) fn parse_minute_second < T : EncodingType > (cursor : & mut Cursor < T > , is_leap_second_valid : bool ,) -> ParserResult < u8 > { let (valid_range , err) = if is_leap_second_valid { (0 ..= 60 , ParseError :: TimeSecond) } else { (0 ..= 59 , ParseError :: TimeMinuteSecond) } ; let first = cursor . next_digit () ? . ok_or (err) ? ; let min_sec_value = first * 10 + cursor . next_digit () ? . ok_or (err) ? ; if ! valid_range . contains (& min_sec_value) { return Err (err) ; } Ok (min_sec_value) }
};
}
