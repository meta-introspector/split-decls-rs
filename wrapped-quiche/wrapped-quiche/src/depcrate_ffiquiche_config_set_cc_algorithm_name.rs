// Generated macro for quiche_config_set_cc_algorithm_name (function)
macro_rules! Depcrate_ffiquiche_config_set_cc_algorithm_name {
() => {
// Module: crate::ffi
// Provides: {"quiche_config_set_cc_algorithm_name"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_config_set_cc_algorithm_name (config : & mut Config , name : * const c_char ,) -> c_int { let name = unsafe { ffi :: CStr :: from_ptr (name) . to_str () . unwrap () } ; match config . set_cc_algorithm_name (name) { Ok (_) => 0 , Err (e) => e . to_c () as c_int , } }
};
}
