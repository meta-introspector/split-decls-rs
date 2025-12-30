// Generated macro for ParserConfig (struct)
macro_rules! DepcrateParserConfig {
() => {
// Module: crate
// Provides: {"ParserConfig"}
// Dependencies: {}
# [doc = " Parser configuration."] # [derive (Clone , Debug , Default)] pub struct ParserConfig { allow_spaces_after_header_name_in_responses : bool , allow_obsolete_multiline_headers_in_responses : bool , allow_multiple_spaces_in_request_line_delimiters : bool , allow_multiple_spaces_in_response_status_delimiters : bool , allow_space_before_first_header_name : bool , ignore_invalid_headers_in_responses : bool , ignore_invalid_headers_in_requests : bool , }
};
}
