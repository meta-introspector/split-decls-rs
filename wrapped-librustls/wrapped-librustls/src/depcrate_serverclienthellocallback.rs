// Generated macro for ClientHelloCallback (type)
macro_rules! Depcrate_serverClientHelloCallback {
() => {
// Module: crate::server
// Provides: {"ClientHelloCallback"}
// Dependencies: {}
type ClientHelloCallback = unsafe extern "C" fn (userdata : rustls_client_hello_userdata , hello : * const rustls_client_hello ,) -> * const rustls_certified_key ;
};
}
