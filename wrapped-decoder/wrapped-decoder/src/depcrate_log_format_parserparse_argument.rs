// Generated macro for parse_argument (function)
macro_rules! Depcrate_log_format_parserparse_argument {
() => {
// Module: crate::log::format::parser
// Provides: {"parse_argument"}
// Dependencies: {}
fn parse_argument < const NEST : bool > (input : & str) -> IResult < & str , LogSegment , () > { let take_between_matching_brackets = delimited (char ('{') , take_until_unbalanced ('{' , '}') , char ('}')) ; take_between_matching_brackets . and_then (parse_log_segment :: < NEST >) . parse (input) }
};
}
