// Generated macro for quiche_config_load_verify_locations_from_file (function)
macro_rules! Depcrate_ffiquiche_config_load_verify_locations_from_file {
() => {
// Module: crate::ffi
// Provides: {"quiche_config_load_verify_locations_from_file"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_config_load_verify_locations_from_file (config : & mut Config , path : * const c_char ,) -> c_int { let path = unsafe { ffi :: CStr :: from_ptr (path) . to_str () . unwrap () } ; match config . load_verify_locations_from_file (path) { Ok (_) => 0 , Err (e) => e . to_c () as c_int , } }
};
}
