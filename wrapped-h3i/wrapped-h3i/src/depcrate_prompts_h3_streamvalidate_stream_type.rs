// Generated macro for validate_stream_type (function)
macro_rules! Depcrate_prompts_h3_streamvalidate_stream_type {
() => {
// Module: crate::prompts::h3::stream
// Provides: {"validate_stream_type"}
// Dependencies: {}
fn validate_stream_type (id : & str) -> SuggestionResult < Validation > { if matches ! (id , CONTROL_STREAM | PUSH_STREAM | QPACK_ENCODER | QPACK_DECODER) { return Ok (Validation :: Valid) ; } h3 :: validate_varint (id) }
};
}
