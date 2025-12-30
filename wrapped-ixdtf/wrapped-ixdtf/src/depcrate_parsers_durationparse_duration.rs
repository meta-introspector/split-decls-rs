// Generated macro for parse_duration (function)
macro_rules! Depcrate_parsers_durationparse_duration {
() => {
// Module: crate::parsers::duration
// Provides: {"parse_duration"}
// Dependencies: {}
pub (crate) fn parse_duration < T : EncodingType > (cursor : & mut Cursor < T > ,) -> ParserResult < DurationParseRecord > { let sign = if cursor . check (is_ascii_sign) ? . ok_or_else (| | ParseError :: abrupt_end ("DurationStart")) ? { cursor . next_or (ParseError :: ImplAssert) ? == b'+' } else { true } ; assert_syntax ! (is_duration_designator (cursor . next_or (ParseError :: abrupt_end ("DurationDesignator")) ?) , DurationDisgnator ,) ; let date = if cursor . check (is_time_designator) ? . ok_or (ParseError :: abrupt_end ("Duration")) ? { None } else { Some (parse_date_duration (cursor) ?) } ; let time = parse_time_duration (cursor) ? ; cursor . close () ? ; Ok (DurationParseRecord { sign : sign . into () , date , time , }) }
};
}
