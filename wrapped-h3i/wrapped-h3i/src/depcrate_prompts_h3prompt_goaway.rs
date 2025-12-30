// Generated macro for prompt_goaway (function)
macro_rules! Depcrate_prompts_h3prompt_goaway {
() => {
// Module: crate::prompts::h3
// Provides: {"prompt_goaway"}
// Dependencies: {}
fn prompt_goaway () -> InquireResult < Action > { let stream_id = h3 :: prompt_stream_id () ? ; let id = h3 :: prompt_varint ("ID:") ? ; let fin_stream = prompt_fin_stream () ? ; let action = Action :: SendFrame { stream_id , fin_stream , frame : quiche :: h3 :: frame :: Frame :: GoAway { id } , } ; Ok (action) }
};
}
