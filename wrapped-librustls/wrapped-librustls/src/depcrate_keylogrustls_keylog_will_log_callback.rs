// Generated macro for rustls_keylog_will_log_callback (type)
macro_rules! Depcrate_keylogrustls_keylog_will_log_callback {
() => {
// Module: crate::keylog
// Provides: {"rustls_keylog_will_log_callback"}
// Dependencies: {}
# [doc = " An optional callback for deciding if key material will be logged."] # [doc = ""] # [doc = " See the documentation on `rustls_client_config_builder_set_key_log` and"] # [doc = " `rustls_server_config_builder_set_key_log` for more information about the"] # [doc = " lifetimes of the parameters."] pub type rustls_keylog_will_log_callback = Option < unsafe extern "C" fn (label : rustls_str) -> c_int > ;
};
}
