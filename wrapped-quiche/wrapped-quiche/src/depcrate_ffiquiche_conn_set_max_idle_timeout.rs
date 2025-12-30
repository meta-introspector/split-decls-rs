// Generated macro for quiche_conn_set_max_idle_timeout (function)
macro_rules! Depcrate_ffiquiche_conn_set_max_idle_timeout {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_set_max_idle_timeout"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_set_max_idle_timeout (conn : & mut Connection , v : u64 ,) -> c_int { match conn . set_max_idle_timeout (v) { Ok (()) => 0 , Err (e) => e . to_c () as c_int , } }
};
}
