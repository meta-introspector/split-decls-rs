// Generated macro for send_headers_frame (function)
macro_rules! Depcrate_actions_h3send_headers_frame {
() => {
// Module: crate::actions::h3
// Provides: {"send_headers_frame"}
// Dependencies: {}
# [doc = " Convenience to convert between header-related data and a"] # [doc = " [Action::SendHeadersFrame]."] pub fn send_headers_frame (stream_id : u64 , fin_stream : bool , headers : Vec < Header > ,) -> Action { let header_block = encode_header_block (& headers) . unwrap () ; Action :: SendHeadersFrame { stream_id , fin_stream , headers , literal_headers : false , frame : Frame :: Headers { header_block } , } }
};
}
