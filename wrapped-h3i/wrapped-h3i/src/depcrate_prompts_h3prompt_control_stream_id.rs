// Generated macro for prompt_control_stream_id (function)
macro_rules! Depcrate_prompts_h3prompt_control_stream_id {
() => {
// Module: crate::prompts::h3
// Provides: {"prompt_control_stream_id"}
// Dependencies: {}
fn prompt_control_stream_id () -> InquireResult < u64 > { let id = Text :: new (STREAM_ID_PROMPT) . with_validator (h3 :: validate_varint) . with_autocomplete (& control_stream_suggestor) . with_help_message (ESC_TO_RET) . prompt () ? ; Ok (id . parse :: < u64 > () . unwrap ()) }
};
}
