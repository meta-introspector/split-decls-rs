// Generated macro for prompt_close_stream (function)
macro_rules! Depcrate_prompts_h3_streamprompt_close_stream {
() => {
// Module: crate::prompts::h3::stream
// Provides: {"prompt_close_stream"}
// Dependencies: {}
fn prompt_close_stream () -> InquireResult < (u64 , u64) > { let id = h3 :: prompt_stream_id () ? ; let (_ , error_code) = prompt_transport_or_app_error () ? ; Ok ((id , error_code)) }
};
}
