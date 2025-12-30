// Generated macro for parse_date_year (function)
macro_rules! Depcrate_parsers_datetimeparse_date_year {
() => {
// Module: crate::parsers::datetime
// Provides: {"parse_date_year"}
// Dependencies: {}
# [inline] fn parse_date_year < T : EncodingType > (cursor : & mut Cursor < T >) -> ParserResult < i32 > { if cursor . check_or (false , is_ascii_sign) ? { let sign = if cursor . next_or (ParseError :: ImplAssert) ? == b'+' { 1 } else { - 1 } ; let first = cursor . next_digit () ? . ok_or (ParseError :: DateExtendedYear) ? as i32 * 100_000 ; let second = cursor . next_digit () ? . ok_or (ParseError :: DateExtendedYear) ? as i32 * 10_000 ; let third = cursor . next_digit () ? . ok_or (ParseError :: DateExtendedYear) ? as i32 * 1000 ; let fourth = cursor . next_digit () ? . ok_or (ParseError :: DateExtendedYear) ? as i32 * 100 ; let fifth = cursor . next_digit () ? . ok_or (ParseError :: DateExtendedYear) ? as i32 * 10 ; let year_value = first + second + third + fourth + fifth + cursor . next_digit () ? . ok_or (ParseError :: DateExtendedYear) ? as i32 ; if sign == - 1 && year_value == 0 { return Err (ParseError :: DateExtendedYear) ; } let year = sign * year_value ; return Ok (year) ; } let first = cursor . next_digit () ? . ok_or (ParseError :: DateYear) ? as i32 * 1000 ; let second = cursor . next_digit () ? . ok_or (ParseError :: DateYear) ? as i32 * 100 ; let third = cursor . next_digit () ? . ok_or (ParseError :: DateYear) ? as i32 * 10 ; let year_value = first + second + third + cursor . next_digit () ? . ok_or (ParseError :: DateYear) ? as i32 ; Ok (year_value) }
};
}
