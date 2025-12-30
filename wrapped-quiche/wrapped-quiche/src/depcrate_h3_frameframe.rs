// Generated macro for Frame (enum)
macro_rules! Depcrate_h3_frameFrame {
() => {
// Module: crate::h3::frame
// Provides: {"Frame"}
// Dependencies: {}
# [derive (Clone , PartialEq , Eq)] pub enum Frame { Data { payload : Vec < u8 > , } , Headers { header_block : Vec < u8 > , } , CancelPush { push_id : u64 , } , Settings { max_field_section_size : Option < u64 > , qpack_max_table_capacity : Option < u64 > , qpack_blocked_streams : Option < u64 > , connect_protocol_enabled : Option < u64 > , h3_datagram : Option < u64 > , grease : Option < (u64 , u64) > , additional_settings : Option < Vec < (u64 , u64) > > , raw : Option < Vec < (u64 , u64) > > , } , PushPromise { push_id : u64 , header_block : Vec < u8 > , } , GoAway { id : u64 , } , MaxPushId { push_id : u64 , } , PriorityUpdateRequest { prioritized_element_id : u64 , priority_field_value : Vec < u8 > , } , PriorityUpdatePush { prioritized_element_id : u64 , priority_field_value : Vec < u8 > , } , Unknown { raw_type : u64 , payload : Vec < u8 > , } , }
};
}
