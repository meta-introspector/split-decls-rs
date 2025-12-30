// Generated macro for impl_60 (impl)
macro_rules! Depcrate_client_legacy_clientimpl_60 {
() => {
// Module: crate::client::legacy::client
// Provides: {"impl_60"}
// Dependencies: {}
impl StdError for Error { fn source (& self) -> Option < & (dyn StdError + 'static) > { self . source . as_ref () . map (| e | & * * e as _) } }
};
}
