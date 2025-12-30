// Generated macro for transport_error_code_suggestor (function)
macro_rules! Depcrate_prompts_h3_errorstransport_error_code_suggestor {
() => {
// Module: crate::prompts::h3::errors
// Provides: {"transport_error_code_suggestor"}
// Dependencies: {}
fn transport_error_code_suggestor (val : & str ,) -> Result < Vec < String > , CustomUserError > { let suggestions = [NO_ERROR , INTERNAL_ERROR , CONNECTION_REFUSED , FLOW_CONTROL_ERROR , STREAM_LIMIT_ERROR , STREAM_STATE_ERROR , FINAL_SIZE_ERROR , FRAME_ENCODING_ERROR , TRANSPORT_PARAMETER_ERROR , CONNECTION_ID_LIMIT_ERROR , PROTOCOL_VIOLATION , INVALID_TOKEN , APPLICATION_ERROR , CRYPTO_BUFFER_EXCEEDED , KEY_UPDATE_ERROR , AEAD_LIMIT_REACHED , NO_VIABLE_PATH , VERSION_NEGOTIATION_ERROR ,] ; super :: squish_suggester (& suggestions , val) }
};
}
