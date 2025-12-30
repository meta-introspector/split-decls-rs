// Generated macro for impl_85 (impl)
macro_rules! Depcrate_h2impl_85 {
() => {
// Module: crate::h2
// Provides: {"impl_85"}
// Dependencies: {}
impl Http2Settings { pub fn set_from_wire (& mut self , id : u16 , value : u32) { match id { H2_HEADER_TABLE_SIZE => self . header_table_size = Some (value) , H2_ENABLE_PUSH => self . enable_push = Some (value) , H2_MAX_CONCURRENT_STREAMS => self . max_concurrent_streams = Some (value) , H2_INITIAL_WINDOW_SIZE => self . initial_window_size = Some (value) , H2_MAX_FRAME_SIZE => self . max_frame_size = Some (value) , H2_MAX_HEADER_LIST_SIZE => self . max_header_list_size = Some (value) , H2_ENABLE_CONNECT_PROTOCOL => self . enable_connect_protocol = Some (value) , H2_NO_RFC7540_PRIORITIES => self . no_rfc7540_priorities = Some (value) , H2_TLS_RENEG_PERMITTED => self . tls_reneg_permitted = Some (value) , H2_ENABLE_METADATA => self . enable_metadata = Some (value) , _ => () , } } }
};
}
