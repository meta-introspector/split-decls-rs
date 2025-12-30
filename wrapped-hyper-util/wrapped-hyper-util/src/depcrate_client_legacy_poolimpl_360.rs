// Generated macro for impl_360 (impl)
macro_rules! Depcrate_client_legacy_poolimpl_360 {
() => {
// Module: crate::client::legacy::pool
// Provides: {"impl_360"}
// Dependencies: {}
impl < T : Poolable , K : Key > Pooled < T , K > { pub fn is_reused (& self) -> bool { self . is_reused } pub fn is_pool_enabled (& self) -> bool { self . pool . 0 . is_some () } fn as_ref (& self) -> & T { self . value . as_ref () . expect ("not dropped") } fn as_mut (& mut self) -> & mut T { self . value . as_mut () . expect ("not dropped") } }
};
}
