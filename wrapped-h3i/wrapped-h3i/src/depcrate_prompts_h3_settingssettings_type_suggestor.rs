// Generated macro for settings_type_suggestor (function)
macro_rules! Depcrate_prompts_h3_settingssettings_type_suggestor {
() => {
// Module: crate::prompts::h3::settings
// Provides: {"settings_type_suggestor"}
// Dependencies: {}
fn settings_type_suggestor (val : & str) -> SuggestionResult < Vec < String > > { let suggestions = [QPACK_MAX_TABLE_CAPACITY , MAX_FIELD_SECTION_SIZE , QPACK_BLOCKED_STREAMS , ENABLE_CONNECT_PROTOCOL , H3_DATAGRAM ,] ; squish_suggester (& suggestions , val) }
};
}
