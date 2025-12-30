// Generated macro for quiche_h3_config_new (function)
macro_rules! Depcrate_h3_ffiquiche_h3_config_new {
() => {
// Module: crate::h3::ffi
// Provides: {"quiche_h3_config_new"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_h3_config_new () -> * mut h3 :: Config { match h3 :: Config :: new () { Ok (c) => Box :: into_raw (Box :: new (c)) , Err (_) => ptr :: null_mut () , } }
};
}
