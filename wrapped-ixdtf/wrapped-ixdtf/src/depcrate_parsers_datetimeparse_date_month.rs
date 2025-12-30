// Generated macro for parse_date_month (function)
macro_rules! Depcrate_parsers_datetimeparse_date_month {
() => {
// Module: crate::parsers::datetime
// Provides: {"parse_date_month"}
// Dependencies: {}
# [inline] fn parse_date_month < T : EncodingType > (cursor : & mut Cursor < T >) -> ParserResult < u8 > { let first = cursor . next_digit () ? . ok_or (ParseError :: DateMonth) ? ; let month_value = first * 10 + cursor . next_digit () ? . ok_or (ParseError :: DateMonth) ? ; if ! (1 ..= 12) . contains (& month_value) { return Err (ParseError :: InvalidMonthRange) ; } Ok (month_value) }
};
}
