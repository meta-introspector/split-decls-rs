// Generated macro for impl_776 (impl)
macro_rules! Depcrate_h3_frameimpl_776 {
() => {
// Module: crate::h3::frame
// Provides: {"impl_776"}
// Dependencies: {}
impl std :: fmt :: Debug for Frame { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { match self { Frame :: Data { .. } => { write ! (f , "DATA") ? ; } , Frame :: Headers { .. } => { write ! (f , "HEADERS") ? ; } , Frame :: CancelPush { push_id } => { write ! (f , "CANCEL_PUSH push_id={push_id}") ? ; } , Frame :: Settings { max_field_section_size , qpack_max_table_capacity , qpack_blocked_streams , additional_settings , raw , .. } => { write ! (f , "SETTINGS max_field_section={max_field_section_size:?}, qpack_max_table={qpack_max_table_capacity:?}, qpack_blocked={qpack_blocked_streams:?} raw={raw:?}, additional_settings={additional_settings:?}") ? ; } , Frame :: PushPromise { push_id , header_block , } => { write ! (f , "PUSH_PROMISE push_id={} len={}" , push_id , header_block . len ()) ? ; } , Frame :: GoAway { id } => { write ! (f , "GOAWAY id={id}") ? ; } , Frame :: MaxPushId { push_id } => { write ! (f , "MAX_PUSH_ID push_id={push_id}") ? ; } , Frame :: PriorityUpdateRequest { prioritized_element_id , priority_field_value , } => { write ! (f , "PRIORITY_UPDATE request_stream_id={}, priority_field_len={}" , prioritized_element_id , priority_field_value . len ()) ? ; } , Frame :: PriorityUpdatePush { prioritized_element_id , priority_field_value , } => { write ! (f , "PRIORITY_UPDATE push_id={}, priority_field_len={}" , prioritized_element_id , priority_field_value . len ()) ? ; } , Frame :: Unknown { raw_type , .. } => { write ! (f , "UNKNOWN raw_type={raw_type}" ,) ? ; } , } Ok (()) } }
};
}
