// Generated macro for quiche_h3_config_set_max_field_section_size (function)
macro_rules! Depcrate_h3_ffiquiche_h3_config_set_max_field_section_size {
() => {
// Module: crate::h3::ffi
// Provides: {"quiche_h3_config_set_max_field_section_size"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_h3_config_set_max_field_section_size (config : & mut h3 :: Config , v : u64 ,) { config . set_max_field_section_size (v) ; }
};
}
