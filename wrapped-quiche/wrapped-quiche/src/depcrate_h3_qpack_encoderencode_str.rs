// Generated macro for encode_str (function)
macro_rules! Depcrate_h3_qpack_encoderencode_str {
() => {
// Module: crate::h3::qpack::encoder
// Provides: {"encode_str"}
// Dependencies: {}
# [inline] pub fn encode_str < const LOWER_CASE : bool > (v : & [u8] , first : u8 , prefix : usize , b : & mut octets :: OctetsMut ,) -> Result < () > { match octets :: huffman_encoding_len :: < LOWER_CASE > (v) { Ok (len) => { encode_int (len as u64 , first | (1 << prefix) , prefix , b) ? ; b . put_huffman_encoded :: < LOWER_CASE > (v) ? ; } , Err (_) => { encode_int (v . len () as u64 , first , prefix , b) ? ; if LOWER_CASE { b . put_bytes (& v . to_ascii_lowercase ()) ? ; } else { b . put_bytes (v) ? ; } } , } Ok (()) }
};
}
