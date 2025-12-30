// Generated macro for impl_376 (impl)
macro_rules! Depcrate_client_legacy_poolimpl_376 {
() => {
// Module: crate::client::legacy::pool
// Provides: {"impl_376"}
// Dependencies: {}
impl < T : Poolable , K : Key > Drop for Connecting < T , K > { fn drop (& mut self) { if let Some (pool) = self . pool . upgrade () { if let Ok (mut inner) = pool . lock () { inner . connected (& self . key) ; } } } }
};
}
