// Generated macro for to_ascii_byte (function)
macro_rules! Depcrate_coreto_ascii_byte {
() => {
// Module: crate::core
// Provides: {"to_ascii_byte"}
// Dependencies: {}
# [inline] fn to_ascii_byte (b : u16) -> ParserResult < u8 > { if ! (0x01 .. 0x7F) . contains (& b) { return Err (ParseError :: NonAsciiCodePoint) ; } Ok (b as u8) }
};
}
