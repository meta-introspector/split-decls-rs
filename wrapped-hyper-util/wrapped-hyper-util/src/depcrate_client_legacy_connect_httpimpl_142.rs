// Generated macro for impl_142 (impl)
macro_rules! Depcrate_client_legacy_connect_httpimpl_142 {
() => {
// Module: crate::client::legacy::connect::http
// Provides: {"impl_142"}
// Dependencies: {}
impl < T > Connection for TokioIo < T > where T : Connection , { fn connected (& self) -> Connected { self . inner () . connected () } }
};
}
