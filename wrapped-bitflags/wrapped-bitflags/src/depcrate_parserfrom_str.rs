// Generated macro for from_str (function)
macro_rules! Depcrate_parserfrom_str {
() => {
// Module: crate::parser
// Provides: {"from_str"}
// Dependencies: {}
# [doc = "\nParse a flags value from text.\n\nThis function will fail on any names that don't correspond to defined flags.\nUnknown bits will be retained.\n"] pub fn from_str < B : Flags > (input : & str) -> Result < B , ParseError > where B :: Bits : ParseHex , { let mut parsed_flags = B :: empty () ; if input . trim () . is_empty () { return Ok (parsed_flags) ; } for flag in input . split ('|') { let flag = flag . trim () ; if flag . is_empty () { return Err (ParseError :: empty_flag ()) ; } let parsed_flag = if let Some (flag) = flag . strip_prefix ("0x") { let bits = < B :: Bits > :: parse_hex (flag) . map_err (| _ | ParseError :: invalid_hex_flag (flag)) ? ; B :: from_bits_retain (bits) } else { B :: from_name (flag) . ok_or_else (| | ParseError :: invalid_named_flag (flag)) ? } ; parsed_flags . insert (parsed_flag) ; } Ok (parsed_flags) }
};
}
