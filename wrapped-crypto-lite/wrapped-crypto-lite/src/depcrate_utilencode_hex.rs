// Generated macro for encode_hex (function)
macro_rules! Depcrate_utilencode_hex {
() => {
// Module: crate::util
// Provides: {"encode_hex"}
// Dependencies: {}
# [must_use] pub fn encode_hex (bytes : & [u8]) -> String { use core :: fmt :: Write ; let mut s = String :: with_capacity (bytes . len () * 2) ; for byte in bytes { write ! (s , "{byte:02X}") . unwrap () ; } s }
};
}
