// Generated macro for impl_363 (impl)
macro_rules! Depcrate_client_legacy_poolimpl_363 {
() => {
// Module: crate::client::legacy::pool
// Provides: {"impl_363"}
// Dependencies: {}
impl < T : Poolable , K : Key > Drop for Pooled < T , K > { fn drop (& mut self) { if let Some (value) = self . value . take () { if ! value . is_open () { return ; } if let Some (pool) = self . pool . upgrade () { if let Ok (mut inner) = pool . lock () { inner . put (self . key . clone () , value , & pool) ; } } else if ! value . can_share () { trace ! ("pool dropped, dropping pooled ({:?})" , self . key) ; } } } }
};
}
