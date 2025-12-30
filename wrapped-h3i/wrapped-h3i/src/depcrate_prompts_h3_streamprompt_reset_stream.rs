// Generated macro for prompt_reset_stream (function)
macro_rules! Depcrate_prompts_h3_streamprompt_reset_stream {
() => {
// Module: crate::prompts::h3::stream
// Provides: {"prompt_reset_stream"}
// Dependencies: {}
pub fn prompt_reset_stream () -> InquireResult < Action > { let (stream_id , error_code) = prompt_close_stream () ? ; Ok (Action :: ResetStream { stream_id , error_code , }) }
};
}
