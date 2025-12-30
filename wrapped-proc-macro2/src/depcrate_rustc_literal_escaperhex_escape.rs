// Generated macro for hex_escape (function)
macro_rules! Depcrate_rustc_literal_escaperhex_escape {
() => {
// Module: crate::rustc_literal_escaper
// Provides: {"hex_escape"}
// Dependencies: {}
# [doc = " Interpret a hexadecimal escape"] # [doc = ""] # [doc = " Parses the two hexadecimal characters of a hexadecimal escape without the leading r\"\\x\"."] # [inline] fn hex_escape (chars : & mut impl Iterator < Item = char >) -> Result < u8 , EscapeError > { let hi = chars . next () . ok_or (EscapeError :: TooShortHexEscape) ? ; let hi = hi . to_digit (16) . ok_or (EscapeError :: InvalidCharInHexEscape) ? ; let lo = chars . next () . ok_or (EscapeError :: TooShortHexEscape) ? ; let lo = lo . to_digit (16) . ok_or (EscapeError :: InvalidCharInHexEscape) ? ; Ok ((hi * 16 + lo) as u8) }
};
}
