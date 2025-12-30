// Generated macro for quiche_config_free (function)
macro_rules! Depcrate_ffiquiche_config_free {
() => {
// Module: crate::ffi
// Provides: {"quiche_config_free"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_config_free (config : * mut Config) { drop (unsafe { Box :: from_raw (config) }) ; }
};
}
