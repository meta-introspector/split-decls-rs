// Generated macro for impl_36 (impl)
macro_rules! Depcrate_client_legacy_clientimpl_36 {
() => {
// Module: crate::client::legacy::client
// Provides: {"impl_36"}
// Dependencies: {}
impl < C : Clone , B > Clone for Client < C , B > { fn clone (& self) -> Client < C , B > { Client { config : self . config , exec : self . exec . clone () , # [cfg (feature = "http1")] h1_builder : self . h1_builder . clone () , # [cfg (feature = "http2")] h2_builder : self . h2_builder . clone () , connector : self . connector . clone () , pool : self . pool . clone () , } } }
};
}
