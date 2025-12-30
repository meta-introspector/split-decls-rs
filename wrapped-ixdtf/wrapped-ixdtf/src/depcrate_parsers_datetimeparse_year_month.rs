// Generated macro for parse_year_month (function)
macro_rules! Depcrate_parsers_datetimeparse_year_month {
() => {
// Module: crate::parsers::datetime
// Provides: {"parse_year_month"}
// Dependencies: {}
pub (crate) fn parse_year_month < T : EncodingType > (cursor : & mut Cursor < T > ,) -> ParserResult < DateRecord > { let year = parse_date_year (cursor) ? ; cursor . advance_if (cursor . check_or (false , is_hyphen) ?) ; let month = parse_date_month (cursor) ? ; Ok (DateRecord { year , month , day : 1 , }) }
};
}
