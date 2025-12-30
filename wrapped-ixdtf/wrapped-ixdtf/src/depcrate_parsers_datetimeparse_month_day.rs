// Generated macro for parse_month_day (function)
macro_rules! Depcrate_parsers_datetimeparse_month_day {
() => {
// Module: crate::parsers::datetime
// Provides: {"parse_month_day"}
// Dependencies: {}
# [doc = " Parses a `DateSpecMonthDay`"] pub (crate) fn parse_month_day < T : EncodingType > (cursor : & mut Cursor < T >) -> ParserResult < DateRecord > { let hyphenated = cursor . check (is_hyphen) ? . ok_or (ParseError :: abrupt_end ("MonthDay")) ? ; cursor . advance_if (hyphenated) ; let balanced_hyphens = hyphenated && cursor . check (is_hyphen) ? . ok_or (ParseError :: abrupt_end ("MonthDay")) ? ; cursor . advance_if (balanced_hyphens) ; if hyphenated && ! balanced_hyphens { return Err (ParseError :: MonthDayHyphen) ; } let month = parse_date_month (cursor) ? ; cursor . advance_if (cursor . check_or (false , is_hyphen) ?) ; let day = parse_date_day (cursor) ? ; if ! is_valid_month_day (month , day) { return Err (ParseError :: InvalidMonthDay) ; } Ok (DateRecord { year : 0 , month , day , }) }
};
}
