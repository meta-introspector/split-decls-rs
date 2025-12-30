// Generated macro for from_hex (function)
macro_rules! Depcrate_testutilfrom_hex {
() => {
// Module: crate::testutil
// Provides: {"from_hex"}
// Dependencies: {}
# [doc = " Decode an string of hex digits into a sequence of bytes. The input must"] # [doc = " have an even number of digits."] pub fn from_hex (hex_str : & str) -> Result < Vec < u8 > , String > { if hex_str . len () % 2 != 0 { return Err (String :: from ("Hex string does not have an even number of digits" ,)) ; } let mut result = Vec :: with_capacity (hex_str . len () / 2) ; for digits in hex_str . as_bytes () . chunks (2) { let hi = from_hex_digit (digits [0]) ? ; let lo = from_hex_digit (digits [1]) ? ; result . push ((hi * 0x10) | lo) ; } Ok (result) }
};
}
