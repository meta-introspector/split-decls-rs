// Generated macro for Config (struct)
macro_rules! Depcrate_h3Config {
() => {
// Module: crate::h3
// Provides: {"Config"}
// Dependencies: {}
# [doc = " An HTTP/3 configuration."] pub struct Config { max_field_section_size : Option < u64 > , qpack_max_table_capacity : Option < u64 > , qpack_blocked_streams : Option < u64 > , connect_protocol_enabled : Option < u64 > , # [doc = " additional settings are settings that are not part of the H3"] # [doc = " settings explicitly handled above"] additional_settings : Option < Vec < (u64 , u64) > > , }
};
}
