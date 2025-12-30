// Generated macro for Http3Conn (struct)
macro_rules! Depcrate_commonHttp3Conn {
() => {
// Module: crate::common
// Provides: {"Http3Conn"}
// Dependencies: {}
pub struct Http3Conn { h3_conn : quiche :: h3 :: Connection , reqs_hdrs_sent : usize , reqs_complete : usize , largest_processed_request : u64 , reqs : Vec < Http3Request > , body : Option < Vec < u8 > > , sent_body_bytes : HashMap < u64 , usize > , dump_json : bool , dgram_sender : Option < Http3DgramSender > , output_sink : Rc < RefCell < dyn FnMut (String) > > , }
};
}
