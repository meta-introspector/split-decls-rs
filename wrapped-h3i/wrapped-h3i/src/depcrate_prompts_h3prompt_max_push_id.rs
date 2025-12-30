// Generated macro for prompt_max_push_id (function)
macro_rules! Depcrate_prompts_h3prompt_max_push_id {
() => {
// Module: crate::prompts::h3
// Provides: {"prompt_max_push_id"}
// Dependencies: {}
fn prompt_max_push_id () -> InquireResult < Action > { let stream_id = h3 :: prompt_stream_id () ? ; let push_id = h3 :: prompt_varint (PUSH_ID_PROMPT) ? ; let fin_stream = prompt_fin_stream () ? ; let action = Action :: SendFrame { stream_id , fin_stream , frame : quiche :: h3 :: frame :: Frame :: MaxPushId { push_id } , } ; Ok (action) }
};
}
