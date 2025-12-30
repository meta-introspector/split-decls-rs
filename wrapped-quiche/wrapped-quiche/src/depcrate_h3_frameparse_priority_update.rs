// Generated macro for parse_priority_update (function)
macro_rules! Depcrate_h3_frameparse_priority_update {
() => {
// Module: crate::h3::frame
// Provides: {"parse_priority_update"}
// Dependencies: {}
fn parse_priority_update (frame_type : u64 , payload_length : u64 , b : & mut octets :: Octets ,) -> Result < Frame > { let prioritized_element_id = b . get_varint () ? ; let priority_field_value_length = payload_length - octets :: varint_len (prioritized_element_id) as u64 ; let priority_field_value = b . get_bytes (priority_field_value_length as usize) ? . to_vec () ; match frame_type { PRIORITY_UPDATE_FRAME_REQUEST_TYPE_ID => Ok (Frame :: PriorityUpdateRequest { prioritized_element_id , priority_field_value , }) , PRIORITY_UPDATE_FRAME_PUSH_TYPE_ID => Ok (Frame :: PriorityUpdatePush { prioritized_element_id , priority_field_value , }) , _ => unreachable ! () , } }
};
}
