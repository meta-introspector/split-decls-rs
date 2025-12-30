// Generated macro for parse_datagram_frame (function)
macro_rules! Depcrate_frameparse_datagram_frame {
() => {
// Module: crate::frame
// Provides: {"parse_datagram_frame"}
// Dependencies: {}
fn parse_datagram_frame (ty : u64 , b : & mut octets :: Octets) -> Result < Frame > { let first = ty as u8 ; let len = if first & 0x01 != 0 { b . get_varint () ? as usize } else { b . cap () } ; let data = b . get_bytes (len) ? ; Ok (Frame :: Datagram { data : Vec :: from (data . buf ()) , }) }
};
}
