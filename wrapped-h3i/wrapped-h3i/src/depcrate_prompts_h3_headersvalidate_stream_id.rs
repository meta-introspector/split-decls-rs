// Generated macro for validate_stream_id (function)
macro_rules! Depcrate_prompts_h3_headersvalidate_stream_id {
() => {
// Module: crate::prompts::h3::headers
// Provides: {"validate_stream_id"}
// Dependencies: {}
fn validate_stream_id (id : & str) -> SuggestionResult < Validation > { if id . is_empty () { return Ok (Validation :: Valid) ; } h3 :: validate_varint (id) }
};
}
