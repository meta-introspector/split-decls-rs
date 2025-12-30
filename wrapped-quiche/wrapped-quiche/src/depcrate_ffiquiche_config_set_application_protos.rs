// Generated macro for quiche_config_set_application_protos (function)
macro_rules! Depcrate_ffiquiche_config_set_application_protos {
() => {
// Module: crate::ffi
// Provides: {"quiche_config_set_application_protos"}
// Dependencies: {}
# [no_mangle] # [doc = " Corresponds to the `Config::set_application_protos_wire_format` Rust"] # [doc = " function."] pub extern "C" fn quiche_config_set_application_protos (config : & mut Config , protos : * const u8 , protos_len : size_t ,) -> c_int { let protos = unsafe { slice :: from_raw_parts (protos , protos_len) } ; match config . set_application_protos_wire_format (protos) { Ok (_) => 0 , Err (e) => e . to_c () as c_int , } }
};
}
