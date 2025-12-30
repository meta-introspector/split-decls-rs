// Generated macro for from_hex_digit (function)
macro_rules! Depcrate_testutilfrom_hex_digit {
() => {
// Module: crate::testutil
// Provides: {"from_hex_digit"}
// Dependencies: {}
fn from_hex_digit (d : u8) -> Result < u8 , String > { use core :: ops :: RangeInclusive ; const DECIMAL : (u8 , RangeInclusive < u8 >) = (0 , b'0' ..= b'9') ; const HEX_LOWER : (u8 , RangeInclusive < u8 >) = (10 , b'a' ..= b'f') ; const HEX_UPPER : (u8 , RangeInclusive < u8 >) = (10 , b'A' ..= b'F') ; for (offset , range) in & [DECIMAL , HEX_LOWER , HEX_UPPER] { if range . contains (& d) { return Ok (d - range . start () + offset) ; } } Err (format ! ("Invalid hex digit '{}'" , d as char)) }
};
}
