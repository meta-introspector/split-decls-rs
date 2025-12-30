// Generated macro for parse_nested (function)
macro_rules! Depcrate_log_format_parserparse_nested {
() => {
// Module: crate::log::format::parser
// Provides: {"parse_nested"}
// Dependencies: {}
fn parse_nested < const NEST : bool > (input : & str) -> IResult < & str , IntermediateOutput , () > { let parse_nested_argument = map_res (parse_argument :: < NEST > , | result | { Ok :: < IntermediateOutput , nom :: Err < () > > (IntermediateOutput :: NestedLogSegment (result)) }) ; let parse_nested_string_segment = map_res (parse_string_segment , | result | { Ok :: < IntermediateOutput , nom :: Err < () > > (IntermediateOutput :: NestedLogSegment (result)) }) ; let parse_nested_format = preceded (char ('%') , parse_format :: < true >) ; let mut parse_all = many0 (alt ((parse_nested_argument , parse_nested_string_segment , parse_nested_format ,))) ; let (new_input , output) = parse_all (input) ? ; let log_segment = build_log_segment :: < true > (output) ? ; Ok ((new_input , IntermediateOutput :: NestedLogSegment (log_segment))) }
};
}
