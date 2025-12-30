// Generated macro for quiche_config_load_verify_locations_from_directory (function)
macro_rules! Depcrate_ffiquiche_config_load_verify_locations_from_directory {
() => {
// Module: crate::ffi
// Provides: {"quiche_config_load_verify_locations_from_directory"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_config_load_verify_locations_from_directory (config : & mut Config , path : * const c_char ,) -> c_int { let path = unsafe { ffi :: CStr :: from_ptr (path) . to_str () . unwrap () } ; match config . load_verify_locations_from_directory (path) { Ok (_) => 0 , Err (e) => e . to_c () as c_int , } }
};
}
