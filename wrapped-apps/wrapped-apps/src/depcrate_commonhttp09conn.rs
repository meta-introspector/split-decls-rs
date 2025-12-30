// Generated macro for Http09Conn (struct)
macro_rules! Depcrate_commonHttp09Conn {
() => {
// Module: crate::common
// Provides: {"Http09Conn"}
// Dependencies: {}
pub struct Http09Conn { stream_id : u64 , reqs_sent : usize , reqs_complete : usize , reqs : Vec < Http09Request > , output_sink : Rc < RefCell < dyn FnMut (String) > > , }
};
}
