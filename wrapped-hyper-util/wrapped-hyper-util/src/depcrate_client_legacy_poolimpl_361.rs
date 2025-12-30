// Generated macro for impl_361 (impl)
macro_rules! Depcrate_client_legacy_poolimpl_361 {
() => {
// Module: crate::client::legacy::pool
// Provides: {"impl_361"}
// Dependencies: {}
impl < T : Poolable , K : Key > Deref for Pooled < T , K > { type Target = T ; fn deref (& self) -> & T { self . as_ref () } }
};
}
