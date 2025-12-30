// Generated macro for impl_152 (impl)
macro_rules! Depcrate_client_legacy_connect_httpimpl_152 {
() => {
// Module: crate::client::legacy::connect::http
// Provides: {"impl_152"}
// Dependencies: {}
impl StdError for ConnectError { fn source (& self) -> Option < & (dyn StdError + 'static) > { self . cause . as_ref () . map (| e | & * * e as _) } }
};
}
