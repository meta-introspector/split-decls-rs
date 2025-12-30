// Generated macro for impl_385 (impl)
macro_rules! Depcrate_parseimpl_385 {
() => {
// Module: crate::parse
// Provides: {"impl_385"}
// Dependencies: {}
impl < 'a > ParsedByteStr < 'a > { pub fn try_from_base64 (str : & ParsedStr < 'a >) -> Option < Self > { fn try_decode_base64 (str : & str) -> Option < Vec < u8 > > { const CHARSET : & [u8 ; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/" ; const PADDING : u8 = b'=' ; if (str . len () % 4) != 0 { return None ; } let bstr_no_padding = str . trim_end_matches (char :: from (PADDING)) . as_bytes () ; if (str . len () - bstr_no_padding . len ()) > 2 { return None ; } if bstr_no_padding . contains (& PADDING) { return None ; } if ! str . is_ascii () { return None ; } let mut collected_bits = 0_u8 ; let mut byte_buffer = 0_u16 ; let mut bytes = bstr_no_padding . iter () . copied () ; let mut binary = Vec :: new () ; 'decodeloop : loop { while collected_bits < 8 { if let Some (nextbyte) = bytes . next () { # [allow (clippy :: cast_possible_truncation)] if let Some (idx) = CHARSET . iter () . position (| & x | x == nextbyte) { byte_buffer |= ((idx & 0b0011_1111) as u16) << (10 - collected_bits) ; collected_bits += 6 ; } else { return None ; } } else { break 'decodeloop ; } } binary . push (((0b1111_1111_0000_0000 & byte_buffer) >> 8) as u8) ; byte_buffer &= 0b0000_0000_1111_1111 ; byte_buffer <<= 8 ; collected_bits -= 8 ; } if usize :: from (collected_bits) != ((str . len () - bstr_no_padding . len ()) * 2) { return None ; } Some (binary) } let base64_str = match str { ParsedStr :: Allocated (string) => string . as_str () , ParsedStr :: Slice (str) => str , } ; try_decode_base64 (base64_str) . map (ParsedByteStr :: Allocated) } }
};
}
