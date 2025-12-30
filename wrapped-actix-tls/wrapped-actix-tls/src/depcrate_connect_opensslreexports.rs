// Generated macro for reexports (module)
macro_rules! Depcrate_connect_opensslreexports {
() => {
// Module: crate::connect::openssl
// Provides: {"reexports"}
// Dependencies: {}
pub mod reexports { # ! [doc = " Re-exports from `openssl` and `tokio-openssl` that are useful for connectors."] pub use openssl :: ssl :: { Error , HandshakeError , SslConnector , SslConnectorBuilder , SslMethod } ; pub use tokio_openssl :: SslStream as AsyncSslStream ; }
};
}
