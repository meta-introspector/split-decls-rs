// Generated macro for prompt_headers (function)
macro_rules! Depcrate_prompts_h3_headersprompt_headers {
() => {
// Module: crate::prompts::h3::headers
// Provides: {"prompt_headers"}
// Dependencies: {}
pub fn prompt_headers (sid_alloc : & mut StreamIdAllocator , host_port : & str , raw : bool , literal : bool ,) -> InquireResult < Action > { let stream_id = Text :: new (STREAM_ID_PROMPT) . with_placeholder (EMPTY_PICKS) . with_help_message (ESC_TO_RET) . with_validator (validate_stream_id) . prompt () ? ; let stream_id = match stream_id . as_str () { "" => { let id = sid_alloc . peek_next_id () ; println ! ("{AUTO_PICK}={id}") ; id } , _ => stream_id . parse :: < u64 > () . unwrap () , } ; let mut headers = vec ! [] ; if ! raw { headers . extend_from_slice (& pseudo_headers (host_port) ?) ; } headers . extend_from_slice (& headers_read_loop () ?) ; sid_alloc . take_next_id () ; let header_block = if literal { encode_header_block_literal (& headers) . unwrap_or_default () } else { encode_header_block (& headers) . unwrap_or_default () } ; let fin_stream = prompt_fin_stream () ? ; let action = Action :: SendHeadersFrame { stream_id , fin_stream , headers , literal_headers : literal , frame : Frame :: Headers { header_block } , } ; Ok (action) }
};
}
