// Generated macro for settings_read_loop (function)
macro_rules! Depcrate_prompts_h3_settingssettings_read_loop {
() => {
// Module: crate::prompts::h3::settings
// Provides: {"settings_read_loop"}
// Dependencies: {}
fn settings_read_loop () -> RawSettings { let mut settings = vec ! [] ; loop { let ty = match Text :: new ("setting type:") . with_validator (validate_setting_type) . with_autocomplete (& settings_type_suggestor) . with_help_message ("type 'q!' to stop adding settings") . prompt () { Ok (h) => { if h == "q!" { break ; } h } , Err (_) => { println ! ("An error happened, stopping.") ; break ; } , } ; let ty = match ty . as_str () { QPACK_MAX_TABLE_CAPACITY => 0x1 , MAX_FIELD_SECTION_SIZE => 0x6 , QPACK_BLOCKED_STREAMS => 0x7 , ENABLE_CONNECT_PROTOCOL => 0x8 , H3_DATAGRAM => 0x33 , v => v . parse :: < u64 > () . unwrap () , } ; let value = Text :: new ("setting value:") . with_validator (h3 :: validate_varint) . prompt () . expect ("An error happened, stopping.") . parse :: < u64 > () . unwrap () ; settings . push ((ty , value)) ; } settings }
};
}
