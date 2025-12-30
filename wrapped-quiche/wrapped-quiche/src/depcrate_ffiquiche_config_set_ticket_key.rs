// Generated macro for quiche_config_set_ticket_key (function)
macro_rules! Depcrate_ffiquiche_config_set_ticket_key {
() => {
// Module: crate::ffi
// Provides: {"quiche_config_set_ticket_key"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_config_set_ticket_key (config : & mut Config , key : * const u8 , key_len : size_t ,) -> c_int { let key = unsafe { slice :: from_raw_parts (key , key_len) } ; match config . set_ticket_key (key) { Ok (_) => 0 , Err (e) => e . to_c () as c_int , } }
};
}
