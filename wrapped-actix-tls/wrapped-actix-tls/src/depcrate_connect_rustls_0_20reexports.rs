// Generated macro for reexports (module)
macro_rules! Depcrate_connect_rustls_0_20reexports {
() => {
// Module: crate::connect::rustls_0_20
// Provides: {"reexports"}
// Dependencies: {}
pub mod reexports { # ! [doc = " Re-exports from the `rustls` v0.20 ecosystem that are useful for connectors."] pub use tokio_rustls_023 :: { client :: TlsStream as AsyncTlsStream , rustls :: ClientConfig } ; # [cfg (feature = "rustls-0_20-webpki-roots")] pub use webpki_roots_022 :: TLS_SERVER_ROOTS ; }
};
}
