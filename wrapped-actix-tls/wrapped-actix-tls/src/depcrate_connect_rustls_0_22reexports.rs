// Generated macro for reexports (module)
macro_rules! Depcrate_connect_rustls_0_22reexports {
() => {
// Module: crate::connect::rustls_0_22
// Provides: {"reexports"}
// Dependencies: {}
pub mod reexports { # ! [doc = " Re-exports from the `rustls` v0.22 ecosystem that are useful for connectors."] pub use tokio_rustls_025 :: { client :: TlsStream as AsyncTlsStream , rustls :: ClientConfig } ; # [cfg (feature = "rustls-0_22-webpki-roots")] pub use webpki_roots_026 :: TLS_SERVER_ROOTS ; }
};
}
