// Generated macro for parse (function)
macro_rules! Depcrate_log_format_parserparse {
() => {
// Module: crate::log::format::parser
// Provides: {"parse"}
// Dependencies: {}
pub (super) fn parse (input : & str) -> Result < Vec < LogSegment > , String > { let mut parse_all = many0 (alt ((parse_argument :: < false > , parse_string_segment))) ; let result = parse_all (input) . map (| (_ , output) | output) . map_err (| e | e . to_string ()) ? ; if ! format_contains_log_specifier (& result) { return Err ("log format must contain a `{s}` format specifier" . to_string ()) ; } Ok (result) }
};
}
