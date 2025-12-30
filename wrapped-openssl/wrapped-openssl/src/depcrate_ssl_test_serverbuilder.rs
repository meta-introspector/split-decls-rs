// Generated macro for Builder (struct)
macro_rules! Depcrate_ssl_test_serverBuilder {
() => {
// Module: crate::ssl::test::server
// Provides: {"Builder"}
// Dependencies: {}
pub struct Builder { ctx : SslContextBuilder , ssl_cb : Box < dyn FnMut (& mut SslRef) + Send > , io_cb : Box < dyn FnMut (SslStream < TcpStream >) + Send > , should_error : bool , }
};
}
