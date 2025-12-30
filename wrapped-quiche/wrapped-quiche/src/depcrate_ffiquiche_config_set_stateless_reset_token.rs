// Generated macro for quiche_config_set_stateless_reset_token (function)
macro_rules! Depcrate_ffiquiche_config_set_stateless_reset_token {
() => {
// Module: crate::ffi
// Provides: {"quiche_config_set_stateless_reset_token"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_config_set_stateless_reset_token (config : & mut Config , v : * const u8 ,) { let reset_token = unsafe { slice :: from_raw_parts (v , 16) } ; let reset_token = match reset_token . try_into () { Ok (rt) => rt , Err (_) => unreachable ! () , } ; let reset_token = u128 :: from_be_bytes (reset_token) ; config . set_stateless_reset_token (Some (reset_token)) ; }
};
}
