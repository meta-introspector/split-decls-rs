// Generated macro for quiche_h3_conn_poll (function)
macro_rules! Depcrate_h3_ffiquiche_h3_conn_poll {
() => {
// Module: crate::h3::ffi
// Provides: {"quiche_h3_conn_poll"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_h3_conn_poll (conn : & mut h3 :: Connection , quic_conn : & mut Connection , ev : * mut * const h3 :: Event ,) -> i64 { match conn . poll (quic_conn) { Ok ((id , v)) => { unsafe { * ev = Box :: into_raw (Box :: new (v)) ; } id as i64 } , Err (e) => e . to_c () as i64 , } }
};
}
