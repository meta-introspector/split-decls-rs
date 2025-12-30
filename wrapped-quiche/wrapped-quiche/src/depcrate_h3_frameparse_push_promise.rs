// Generated macro for parse_push_promise (function)
macro_rules! Depcrate_h3_frameparse_push_promise {
() => {
// Module: crate::h3::frame
// Provides: {"parse_push_promise"}
// Dependencies: {}
fn parse_push_promise (payload_length : u64 , b : & mut octets :: Octets ,) -> Result < Frame > { let push_id = b . get_varint () ? ; let header_block_length = payload_length - octets :: varint_len (push_id) as u64 ; let header_block = b . get_bytes (header_block_length as usize) ? . to_vec () ; Ok (Frame :: PushPromise { push_id , header_block , }) }
};
}
