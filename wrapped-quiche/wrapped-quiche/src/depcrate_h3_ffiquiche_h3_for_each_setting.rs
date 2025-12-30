// Generated macro for quiche_h3_for_each_setting (function)
macro_rules! Depcrate_h3_ffiquiche_h3_for_each_setting {
() => {
// Module: crate::h3::ffi
// Provides: {"quiche_h3_for_each_setting"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_h3_for_each_setting (conn : & h3 :: Connection , cb : extern "C" fn (identifier : u64 , value : u64 , argp : * mut c_void) -> c_int , argp : * mut c_void ,) -> c_int { match conn . peer_settings_raw () { Some (raw) => { for setting in raw { let rc = cb (setting . 0 , setting . 1 , argp) ; if rc != 0 { return rc ; } } 0 } , None => - 1 , } }
};
}
