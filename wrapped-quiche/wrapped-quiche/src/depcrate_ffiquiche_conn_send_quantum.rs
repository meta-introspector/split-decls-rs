// Generated macro for quiche_conn_send_quantum (function)
macro_rules! Depcrate_ffiquiche_conn_send_quantum {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_send_quantum"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_send_quantum (conn : & Connection) -> size_t { conn . send_quantum () as size_t }
};
}
