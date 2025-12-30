// Generated macro for encode (function)
macro_rules! Depcrate_lowerencode {
() => {
// Module: crate::lower
// Provides: {"encode"}
// Dependencies: {}
# [doc = " Encode the input byte slice as lower Base16."] # [doc = ""] # [doc = " Writes the result into the provided destination slice, returning an"] # [doc = " ASCII-encoded lower Base16 (hex) string value."] pub fn encode < 'a > (src : & [u8] , dst : & 'a mut [u8]) -> Result < & 'a [u8] , Error > { let dst = dst . get_mut (.. encoded_len (src)) . ok_or (Error :: InvalidLength) ? ; for (src , dst) in src . iter () . zip (dst . chunks_exact_mut (2)) { dst [0] = encode_nibble (src >> 4) ; dst [1] = encode_nibble (src & 0x0f) ; } Ok (dst) }
};
}
