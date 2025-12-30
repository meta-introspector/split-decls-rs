// Generated macro for validate_h3_error_code (function)
macro_rules! Depcrate_prompts_h3_errorsvalidate_h3_error_code {
() => {
// Module: crate::prompts::h3::errors
// Provides: {"validate_h3_error_code"}
// Dependencies: {}
fn validate_h3_error_code (id : & str) -> SuggestionResult < Validation > { if matches ! (id , H3_NO_ERROR | H3_GENERAL_PROTOCOL_ERROR | H3_INTERNAL_ERROR | H3_STREAM_CREATION_ERROR | H3_CLOSED_CRITICAL_STREAM | H3_FRAME_UNEXPECTED | H3_FRAME_ERROR | H3_EXCESSIVE_LOAD | H3_ID_ERROR | H3_SETTINGS_ERROR | H3_MISSING_SETTINGS | H3_REQUEST_REJECTED | H3_REQUEST_CANCELLED | H3_REQUEST_INCOMPLETE | H3_MESSAGE_ERROR | H3_CONNECT_ERROR | H3_VERSION_FALLBACK | QPACK_DECOMPRESSION_FAILED | QPACK_ENCODER_STREAM_ERROR | QPACK_DECODER_STREAM_ERROR | H3_DATAGRAM_ERROR) { return Ok (Validation :: Valid) ; } h3 :: validate_varint (id) }
};
}
