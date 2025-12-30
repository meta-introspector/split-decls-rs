// Generated macro for validate_transport_error_code (function)
macro_rules! Depcrate_prompts_h3_errorsvalidate_transport_error_code {
() => {
// Module: crate::prompts::h3::errors
// Provides: {"validate_transport_error_code"}
// Dependencies: {}
fn validate_transport_error_code (id : & str ,) -> Result < Validation , CustomUserError > { if matches ! (id , NO_ERROR | INTERNAL_ERROR | CONNECTION_REFUSED | FLOW_CONTROL_ERROR | STREAM_LIMIT_ERROR | STREAM_STATE_ERROR | FINAL_SIZE_ERROR | FRAME_ENCODING_ERROR | TRANSPORT_PARAMETER_ERROR | CONNECTION_ID_LIMIT_ERROR | PROTOCOL_VIOLATION | INVALID_TOKEN | APPLICATION_ERROR | CRYPTO_BUFFER_EXCEEDED | KEY_UPDATE_ERROR | AEAD_LIMIT_REACHED | NO_VIABLE_PATH | VERSION_NEGOTIATION_ERROR) { return Ok (Validation :: Valid) ; } h3 :: validate_varint (id) }
};
}
