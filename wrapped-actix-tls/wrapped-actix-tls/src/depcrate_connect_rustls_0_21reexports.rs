// Generated macro for reexports (module)
macro_rules! Depcrate_connect_rustls_0_21reexports {
() => {
// Module: crate::connect::rustls_0_21
// Provides: {"reexports"}
// Dependencies: {}
pub mod reexports { # ! [doc = " Re-exports from the `rustls` v0.21 ecosystem that are useful for connectors."] pub use tokio_rustls_024 :: { client :: TlsStream as AsyncTlsStream , rustls :: ClientConfig } ; # [cfg (feature = "rustls-0_21-webpki-roots")] pub use webpki_roots_025 :: TLS_SERVER_ROOTS ; }
};
}
