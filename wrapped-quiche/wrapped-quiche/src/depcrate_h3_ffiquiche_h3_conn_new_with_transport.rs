// Generated macro for quiche_h3_conn_new_with_transport (function)
macro_rules! Depcrate_h3_ffiquiche_h3_conn_new_with_transport {
() => {
// Module: crate::h3::ffi
// Provides: {"quiche_h3_conn_new_with_transport"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_h3_conn_new_with_transport (quic_conn : & mut Connection , config : & mut h3 :: Config ,) -> * mut h3 :: Connection { match h3 :: Connection :: with_transport (quic_conn , config) { Ok (c) => Box :: into_raw (Box :: new (c)) , Err (_) => ptr :: null_mut () , } }
};
}
