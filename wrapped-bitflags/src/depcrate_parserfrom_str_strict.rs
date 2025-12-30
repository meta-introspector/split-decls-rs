// Generated macro for from_str_strict (function)
macro_rules! Depcrate_parserfrom_str_strict {
() => {
// Module: crate::parser
// Provides: {"from_str_strict"}
// Dependencies: {}
# [doc = "\nParse a flags value from text.\n\nThis function will fail on any names that don't correspond to defined flags.\nThis function will fail to parse hex values.\n"] pub fn from_str_strict < B : Flags > (input : & str) -> Result < B , ParseError > { let mut parsed_flags = B :: empty () ; if input . trim () . is_empty () { return Ok (parsed_flags) ; } for flag in input . split ('|') { let flag = flag . trim () ; if flag . is_empty () { return Err (ParseError :: empty_flag ()) ; } if flag . starts_with ("0x") { return Err (ParseError :: invalid_hex_flag ("unsupported hex flag value")) ; } let parsed_flag = B :: from_name (flag) . ok_or_else (| | ParseError :: invalid_named_flag (flag)) ? ; parsed_flags . insert (parsed_flag) ; } Ok (parsed_flags) }
};
}
