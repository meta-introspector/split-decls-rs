// Generated macro for encode_stream_header (function)
macro_rules! Depcrate_frameencode_stream_header {
() => {
// Module: crate::frame
// Provides: {"encode_stream_header"}
// Dependencies: {}
pub fn encode_stream_header (stream_id : u64 , offset : u64 , length : u64 , fin : bool , b : & mut octets :: OctetsMut ,) -> Result < () > { let mut ty : u8 = 0x08 ; ty |= 0x04 ; ty |= 0x02 ; if fin { ty |= 0x01 ; } b . put_varint (u64 :: from (ty)) ? ; b . put_varint (stream_id) ? ; b . put_varint (offset) ? ; b . put_varint_with_len (length , 2) ? ; Ok (()) }
};
}
