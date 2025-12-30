// Generated macro for quiche_conn_trace_id (function)
macro_rules! Depcrate_ffiquiche_conn_trace_id {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_trace_id"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_trace_id (conn : & Connection , out : & mut * const u8 , out_len : & mut size_t ,) { let trace_id = conn . trace_id () ; * out = trace_id . as_ptr () ; * out_len = trace_id . len () ; }
};
}
