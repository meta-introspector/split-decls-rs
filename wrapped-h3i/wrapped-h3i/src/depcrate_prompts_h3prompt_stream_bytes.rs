// Generated macro for prompt_stream_bytes (function)
macro_rules! Depcrate_prompts_h3prompt_stream_bytes {
() => {
// Module: crate::prompts::h3
// Provides: {"prompt_stream_bytes"}
// Dependencies: {}
pub fn prompt_stream_bytes () -> InquireResult < Action > { let stream_id = h3 :: prompt_stream_id () ? ; let bytes = Text :: new ("bytes:") . prompt () ? ; let fin_stream = prompt_fin_stream () ? ; Ok (Action :: StreamBytes { stream_id , fin_stream , bytes : bytes . as_bytes () . to_vec () , }) }
};
}
