// Generated macro for quiche_h3_config_set_qpack_blocked_streams (function)
macro_rules! Depcrate_h3_ffiquiche_h3_config_set_qpack_blocked_streams {
() => {
// Module: crate::h3::ffi
// Provides: {"quiche_h3_config_set_qpack_blocked_streams"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_h3_config_set_qpack_blocked_streams (config : & mut h3 :: Config , v : u64 ,) { config . set_qpack_blocked_streams (v) ; }
};
}
