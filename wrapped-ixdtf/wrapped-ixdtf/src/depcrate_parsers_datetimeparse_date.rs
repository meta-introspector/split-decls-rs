// Generated macro for parse_date (function)
macro_rules! Depcrate_parsers_datetimeparse_date {
() => {
// Module: crate::parsers::datetime
// Provides: {"parse_date"}
// Dependencies: {}
# [doc = " Parses `Date` record."] fn parse_date < T : EncodingType > (cursor : & mut Cursor < T >) -> ParserResult < DateRecord > { let year = parse_date_year (cursor) ? ; let hyphenated = cursor . check (is_hyphen) ? . ok_or (ParseError :: abrupt_end ("Date")) ? ; cursor . advance_if (hyphenated) ; let month = parse_date_month (cursor) ? ; let second_hyphen = cursor . check_or (false , is_hyphen) ? ; assert_syntax ! (hyphenated == second_hyphen , DateSeparator) ; cursor . advance_if (second_hyphen) ; let day = parse_date_day (cursor) ? ; check_date_validity (year , month , day) ? ; Ok (DateRecord { year , month , day }) }
};
}
