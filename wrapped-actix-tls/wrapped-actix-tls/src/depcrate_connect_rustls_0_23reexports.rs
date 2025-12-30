// Generated macro for reexports (module)
macro_rules! Depcrate_connect_rustls_0_23reexports {
() => {
// Module: crate::connect::rustls_0_23
// Provides: {"reexports"}
// Dependencies: {}
pub mod reexports { # ! [doc = " Re-exports from the `rustls` v0.23 ecosystem that are useful for connectors."] pub use tokio_rustls_026 :: { client :: TlsStream as AsyncTlsStream , rustls :: ClientConfig } ; # [cfg (feature = "rustls-0_23-webpki-roots")] pub use webpki_roots_026 :: TLS_SERVER_ROOTS ; }
};
}
