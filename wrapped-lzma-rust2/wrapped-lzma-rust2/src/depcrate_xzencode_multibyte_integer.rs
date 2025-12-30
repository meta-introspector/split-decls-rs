// Generated macro for encode_multibyte_integer (function)
macro_rules! Depcrate_xzencode_multibyte_integer {
() => {
// Module: crate::xz
// Provides: {"encode_multibyte_integer"}
// Dependencies: {}
fn encode_multibyte_integer (mut value : u64 , buf : & mut [u8]) -> crate :: Result < usize > { if value > (u64 :: MAX / 2) { return Err (error_invalid_data ("value too big to encode")) ; } let mut i = 0 ; while value >= 0x80 && i < buf . len () { buf [i] = (value as u8) | 0x80 ; value >>= 7 ; i += 1 ; } if i < buf . len () { buf [i] = value as u8 ; i += 1 ; } Ok (i) }
};
}
