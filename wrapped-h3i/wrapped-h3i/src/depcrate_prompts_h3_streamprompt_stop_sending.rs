// Generated macro for prompt_stop_sending (function)
macro_rules! Depcrate_prompts_h3_streamprompt_stop_sending {
() => {
// Module: crate::prompts::h3::stream
// Provides: {"prompt_stop_sending"}
// Dependencies: {}
pub fn prompt_stop_sending () -> InquireResult < Action > { let (stream_id , error_code) = prompt_close_stream () ? ; Ok (Action :: StopSending { stream_id , error_code , }) }
};
}
