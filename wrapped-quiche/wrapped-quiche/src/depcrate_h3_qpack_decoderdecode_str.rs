// Generated macro for decode_str (function)
macro_rules! Depcrate_h3_qpack_decoderdecode_str {
() => {
// Module: crate::h3::qpack::decoder
// Provides: {"decode_str"}
// Dependencies: {}
fn decode_str (b : & mut octets :: Octets) -> Result < Vec < u8 > > { let first = b . peek_u8 () ? ; let huff = first & 0x80 == 0x80 ; let len = decode_int (b , 7) ? as usize ; let mut val = b . get_bytes (len) ? ; let val = if huff { val . get_huffman_decoded () ? } else { val . to_vec () } ; Ok (val) }
};
}
