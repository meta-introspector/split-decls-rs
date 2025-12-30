// Generated macro for parse_log_segment (function)
macro_rules! Depcrate_log_format_parserparse_log_segment {
() => {
// Module: crate::log::format::parser
// Provides: {"parse_log_segment"}
// Dependencies: {}
fn parse_log_segment < const NEST : bool > (input : & str) -> IResult < & str , LogSegment , () > { let (input , output) = if ! NEST { separated_list1 (char (':') , alt ((parse_metadata , parse_format :: < false > , parse_nested :: < true >)) ,) (input) } else { let parse_nested_argument = separated_list1 (char (':') , alt ((parse_metadata , parse_format :: < false >))) ; let parse_nested_log_segment = map_res (parse_nested_argument , | result | { let log_segment = build_log_segment :: < false > (result) ? ; Ok :: < IntermediateOutput , nom :: Err < () > > (IntermediateOutput :: NestedLogSegment (log_segment ,)) }) ; separated_list1 (char ('%') , alt ((parse_nested_log_segment , parse_format :: < false > , parse_nested :: < false > ,)) ,) (input) } ? ; let log_segment = build_log_segment :: < false > (output) ? ; Ok ((input , log_segment)) }
};
}
