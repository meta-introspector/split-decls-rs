// Generated macro for encode (function)
macro_rules! Depcrate_hpack_huffmanencode {
() => {
// Module: crate::hpack::huffman
// Provides: {"encode"}
// Dependencies: {}
pub fn encode (src : & [u8] , dst : & mut BytesMut) { let mut bits : u64 = 0 ; let mut bits_left = 40 ; for & b in src { let (nbits , code) = ENCODE_TABLE [b as usize] ; bits |= code << (bits_left - nbits) ; bits_left -= nbits ; while bits_left <= 32 { dst . put_u8 ((bits >> 32) as u8) ; bits <<= 8 ; bits_left += 8 ; } } if bits_left != 40 { bits |= (1 << bits_left) - 1 ; dst . put_u8 ((bits >> 32) as u8) ; } }
};
}
