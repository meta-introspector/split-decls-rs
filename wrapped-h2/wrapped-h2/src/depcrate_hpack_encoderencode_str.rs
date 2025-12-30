// Generated macro for encode_str (function)
macro_rules! Depcrate_hpack_encoderencode_str {
() => {
// Module: crate::hpack::encoder
// Provides: {"encode_str"}
// Dependencies: {}
fn encode_str (val : & [u8] , dst : & mut BytesMut) { if ! val . is_empty () { let idx = position (dst) ; dst . put_u8 (0) ; huffman :: encode (val , dst) ; let huff_len = position (dst) - (idx + 1) ; if encode_int_one_byte (huff_len , 7) { dst [idx] = 0x80 | huff_len as u8 ; } else { const PLACEHOLDER_LEN : usize = 8 ; let mut buf = [0u8 ; PLACEHOLDER_LEN] ; let head_len = { let mut head_dst = & mut buf [..] ; encode_int (huff_len , 7 , 0x80 , & mut head_dst) ; PLACEHOLDER_LEN - head_dst . remaining_mut () } ; dst . put_slice (& buf [1 .. head_len]) ; for i in 0 .. huff_len { let src_i = idx + 1 + (huff_len - (i + 1)) ; let dst_i = idx + head_len + (huff_len - (i + 1)) ; dst [dst_i] = dst [src_i] ; } for i in 0 .. head_len { dst [idx + i] = buf [i] ; } } } else { dst . put_u8 (0) ; } }
};
}
