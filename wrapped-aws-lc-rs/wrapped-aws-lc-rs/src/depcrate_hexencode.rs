// Generated macro for encode (function)
macro_rules! Depcrate_hexencode {
() => {
// Module: crate::hex
// Provides: {"encode"}
// Dependencies: {}
# [doc = " Converts bytes to a lower-case hex string"] # [allow (clippy :: missing_panics_doc)] pub fn encode < T : AsRef < [u8] > > (bytes : T) -> String { let bytes = bytes . as_ref () ; let mut encoding = String :: with_capacity (2 * bytes . len ()) ; for byte in bytes { let upper_val = byte >> 4u8 ; let lower_val = byte & 0x0f ; encoding . push (char :: from_digit (u32 :: from (upper_val) , 16) . unwrap ()) ; encoding . push (char :: from_digit (u32 :: from (lower_val) , 16) . unwrap ()) ; } encoding }
};
}
