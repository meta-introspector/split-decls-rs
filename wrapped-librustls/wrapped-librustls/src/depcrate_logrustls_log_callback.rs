// Generated macro for rustls_log_callback (type)
macro_rules! Depcrate_logrustls_log_callback {
() => {
// Module: crate::log
// Provides: {"rustls_log_callback"}
// Dependencies: {}
# [doc = " A callback that is invoked for messages logged by rustls."] # [allow (non_camel_case_types)] pub type rustls_log_callback = Option < unsafe extern "C" fn (userdata : * mut c_void , params : * const rustls_log_params) > ;
};
}
