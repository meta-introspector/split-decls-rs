// Generated macro for impl_375 (impl)
macro_rules! Depcrate_client_legacy_poolimpl_375 {
() => {
// Module: crate::client::legacy::pool
// Provides: {"impl_375"}
// Dependencies: {}
impl < T : Poolable , K : Key > Connecting < T , K > { pub fn alpn_h2 (self , pool : & Pool < T , K >) -> Option < Self > { debug_assert ! (self . pool . 0 . is_none () , "Connecting::alpn_h2 but already Http2") ; pool . connecting (& self . key , Ver :: Http2) } }
};
}
