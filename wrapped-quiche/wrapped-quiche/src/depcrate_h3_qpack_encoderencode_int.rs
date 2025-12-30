// Generated macro for encode_int (function)
macro_rules! Depcrate_h3_qpack_encoderencode_int {
() => {
// Module: crate::h3::qpack::encoder
// Provides: {"encode_int"}
// Dependencies: {}
pub fn encode_int (mut v : u64 , first : u8 , prefix : usize , b : & mut octets :: OctetsMut ,) -> Result < () > { let mask = 2u64 . pow (prefix as u32) - 1 ; if v < mask { b . put_u8 (first | v as u8) ? ; return Ok (()) ; } b . put_u8 (first | mask as u8) ? ; v -= mask ; while v >= 128 { b . put_u8 ((v % 128 + 128) as u8) ? ; v >>= 7 ; } b . put_u8 (v as u8) ? ; Ok (()) }
};
}
