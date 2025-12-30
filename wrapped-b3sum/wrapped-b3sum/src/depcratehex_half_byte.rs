// Generated macro for hex_half_byte (function)
macro_rules! Depcratehex_half_byte {
() => {
// Module: crate
// Provides: {"hex_half_byte"}
// Dependencies: {}
fn hex_half_byte (c : char) -> anyhow :: Result < u8 > { if '0' <= c && c <= '9' { return Ok (c as u8 - '0' as u8) ; } if 'a' <= c && c <= 'f' { return Ok (c as u8 - 'a' as u8 + 10) ; } bail ! ("Invalid hex") ; }
};
}
