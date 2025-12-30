// Generated macro for quiche_h3_config_free (function)
macro_rules! Depcrate_h3_ffiquiche_h3_config_free {
() => {
// Module: crate::h3::ffi
// Provides: {"quiche_h3_config_free"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_h3_config_free (config : * mut h3 :: Config) { drop (unsafe { Box :: from_raw (config) }) ; }
};
}
