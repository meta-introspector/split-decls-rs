// Generated macro for send_headers_frame_literal (function)
macro_rules! Depcrate_actions_h3send_headers_frame_literal {
() => {
// Module: crate::actions::h3
// Provides: {"send_headers_frame_literal"}
// Dependencies: {}
# [doc = " Convenience to convert between header-related data and a"] # [doc = " [Action::SendHeadersFrame]. Unlike [`send_headers_frame`],"] # [doc = " this version encodes the headers literally as they are provided,"] # [doc = " not converting the header names to lower-case."] pub fn send_headers_frame_literal (stream_id : u64 , fin_stream : bool , headers : Vec < Header > ,) -> Action { let header_block = encode_header_block_literal (& headers) . unwrap () ; Action :: SendHeadersFrame { stream_id , fin_stream , headers , literal_headers : true , frame : Frame :: Headers { header_block } , } }
};
}
