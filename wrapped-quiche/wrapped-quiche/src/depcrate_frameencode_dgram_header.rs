// Generated macro for encode_dgram_header (function)
macro_rules! Depcrate_frameencode_dgram_header {
() => {
// Module: crate::frame
// Provides: {"encode_dgram_header"}
// Dependencies: {}
pub fn encode_dgram_header (length : u64 , b : & mut octets :: OctetsMut) -> Result < () > { let mut ty : u8 = 0x30 ; ty |= 0x01 ; b . put_varint (u64 :: from (ty)) ? ; b . put_varint_with_len (length , 2) ? ; Ok (()) }
};
}
