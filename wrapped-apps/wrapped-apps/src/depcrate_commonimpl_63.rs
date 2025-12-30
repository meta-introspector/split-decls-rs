// Generated macro for impl_63 (impl)
macro_rules! Depcrate_commonimpl_63 {
() => {
// Module: crate::common
// Provides: {"impl_63"}
// Dependencies: {}
impl Http09Conn { pub fn with_urls (urls : & [url :: Url] , reqs_cardinal : u64 , output_sink : Rc < RefCell < dyn FnMut (String) > > ,) -> Box < dyn HttpConn > { let mut reqs = Vec :: new () ; for url in urls { for i in 1 ..= reqs_cardinal { let request_line = format ! ("GET {}\r\n" , url . path ()) ; reqs . push (Http09Request { url : url . clone () , cardinal : i , request_line , stream_id : None , response_writer : None , }) ; } } let h_conn = Http09Conn { stream_id : 0 , reqs_sent : 0 , reqs_complete : 0 , reqs , output_sink , } ; Box :: new (h_conn) } }
};
}
