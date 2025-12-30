// Generated macro for prompt_push_promise (function)
macro_rules! Depcrate_prompts_h3_headersprompt_push_promise {
() => {
// Module: crate::prompts::h3::headers
// Provides: {"prompt_push_promise"}
// Dependencies: {}
pub fn prompt_push_promise () -> InquireResult < Action > { let stream_id = h3 :: prompt_stream_id () ? ; let push_id = h3 :: prompt_varint (PUSH_ID_PROMPT) ? ; let headers = headers_read_loop () ? ; let header_block = if headers . is_empty () { vec ! [] } else { encode_header_block (& headers) . unwrap () } ; let fin_stream = prompt_fin_stream () ? ; let action = Action :: SendFrame { stream_id , fin_stream , frame : Frame :: PushPromise { push_id , header_block , } , } ; Ok (action) }
};
}
