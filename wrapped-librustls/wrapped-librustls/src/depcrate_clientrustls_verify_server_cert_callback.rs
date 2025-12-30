// Generated macro for rustls_verify_server_cert_callback (type)
macro_rules! Depcrate_clientrustls_verify_server_cert_callback {
() => {
// Module: crate::client
// Provides: {"rustls_verify_server_cert_callback"}
// Dependencies: {}
# [doc = " A callback that is invoked to verify a server certificate."] # [allow (non_camel_case_types)] pub type rustls_verify_server_cert_callback = Option < unsafe extern "C" fn (userdata : rustls_verify_server_cert_user_data , params : * const rustls_verify_server_cert_params ,) -> u32 , > ;
};
}
