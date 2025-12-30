// Generated macro for quiche_config_new (function)
macro_rules! Depcrate_ffiquiche_config_new {
() => {
// Module: crate::ffi
// Provides: {"quiche_config_new"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_config_new (version : u32) -> * mut Config { match Config :: new (version) { Ok (c) => Box :: into_raw (Box :: new (c)) , Err (_) => ptr :: null_mut () , } }
};
}
