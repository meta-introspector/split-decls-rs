// Generated macro for prompt_cancel_push (function)
macro_rules! Depcrate_prompts_h3prompt_cancel_push {
() => {
// Module: crate::prompts::h3
// Provides: {"prompt_cancel_push"}
// Dependencies: {}
fn prompt_cancel_push () -> InquireResult < Action > { let stream_id = h3 :: prompt_stream_id () ? ; let push_id = h3 :: prompt_varint (PUSH_ID_PROMPT) ? ; let fin_stream = prompt_fin_stream () ? ; let action = Action :: SendFrame { stream_id , fin_stream , frame : quiche :: h3 :: frame :: Frame :: CancelPush { push_id } , } ; Ok (action) }
};
}
