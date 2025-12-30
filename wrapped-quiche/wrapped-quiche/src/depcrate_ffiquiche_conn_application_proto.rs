// Generated macro for quiche_conn_application_proto (function)
macro_rules! Depcrate_ffiquiche_conn_application_proto {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_application_proto"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_application_proto (conn : & Connection , out : & mut * const u8 , out_len : & mut size_t ,) { let proto = conn . application_proto () ; * out = proto . as_ptr () ; * out_len = proto . len () ; }
};
}
