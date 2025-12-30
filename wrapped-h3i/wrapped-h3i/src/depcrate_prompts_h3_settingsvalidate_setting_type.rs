// Generated macro for validate_setting_type (function)
macro_rules! Depcrate_prompts_h3_settingsvalidate_setting_type {
() => {
// Module: crate::prompts::h3::settings
// Provides: {"validate_setting_type"}
// Dependencies: {}
fn validate_setting_type (id : & str) -> SuggestionResult < Validation > { if matches ! (id , "q!" | QPACK_MAX_TABLE_CAPACITY | MAX_FIELD_SECTION_SIZE | QPACK_BLOCKED_STREAMS | ENABLE_CONNECT_PROTOCOL | H3_DATAGRAM) { return Ok (Validation :: Valid) ; } h3 :: validate_varint (id) }
};
}
