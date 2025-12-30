// Generated macro for parse_stream_frame (function)
macro_rules! Depcrate_frameparse_stream_frame {
() => {
// Module: crate::frame
// Provides: {"parse_stream_frame"}
// Dependencies: {}
fn parse_stream_frame (ty : u64 , b : & mut octets :: Octets) -> Result < Frame > { let first = ty as u8 ; let stream_id = b . get_varint () ? ; let offset = if first & 0x04 != 0 { b . get_varint () ? } else { 0 } ; let len = if first & 0x02 != 0 { b . get_varint () ? as usize } else { b . cap () } ; if offset + len as u64 >= MAX_STREAM_SIZE { return Err (Error :: InvalidFrame) ; } let fin = first & 0x01 != 0 ; let data = b . get_bytes (len) ? ; let data = < RangeBuf > :: from (data . as_ref () , offset , fin) ; Ok (Frame :: Stream { stream_id , data }) }
};
}
