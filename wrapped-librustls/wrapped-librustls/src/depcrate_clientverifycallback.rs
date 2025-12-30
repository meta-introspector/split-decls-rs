// Generated macro for VerifyCallback (type)
macro_rules! Depcrate_clientVerifyCallback {
() => {
// Module: crate::client
// Provides: {"VerifyCallback"}
// Dependencies: {}
type VerifyCallback = unsafe extern "C" fn (userdata : rustls_verify_server_cert_user_data , params : * const rustls_verify_server_cert_params ,) -> u32 ;
};
}
