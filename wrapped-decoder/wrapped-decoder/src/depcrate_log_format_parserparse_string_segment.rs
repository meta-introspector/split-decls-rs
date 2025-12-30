// Generated macro for parse_string_segment (function)
macro_rules! Depcrate_log_format_parserparse_string_segment {
() => {
// Module: crate::log::format::parser
// Provides: {"parse_string_segment"}
// Dependencies: {}
fn parse_string_segment (input : & str) -> IResult < & str , LogSegment , () > { map (take_till1 (| c | c == '{' || c == '%') , | s : & str | { LogSegment :: new (LogMetadata :: String (s . to_string ())) }) . parse (input) }
};
}
