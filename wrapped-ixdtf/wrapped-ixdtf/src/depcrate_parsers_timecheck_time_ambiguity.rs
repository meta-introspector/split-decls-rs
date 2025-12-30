// Generated macro for check_time_ambiguity (function)
macro_rules! Depcrate_parsers_timecheck_time_ambiguity {
() => {
// Module: crate::parsers::time
// Provides: {"check_time_ambiguity"}
// Dependencies: {}
# [inline] fn check_time_ambiguity < T : EncodingType > (cursor : & mut Cursor < T > , start : usize) -> ParserResult < () > { let current_loc = cursor . pos () ; cursor . set_position (start) ; if parse_month_day (cursor) . is_ok () { return Err (ParseError :: AmbiguousTimeMonthDay) ; } cursor . set_position (start) ; if parse_year_month (cursor) . is_ok () { return Err (ParseError :: AmbiguousTimeYearMonth) ; } cursor . set_position (current_loc) ; Ok (()) }
};
}
