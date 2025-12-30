// Generated macro for parse_json (function)
macro_rules! Depcrate_jsonparse_json {
() => {
// Module: crate::json
// Provides: {"parse_json"}
// Dependencies: {}
pub fn parse_json (input : & str) -> IResult < & str , JsonValue > { ws (json_value) . parse (input) }
};
}
