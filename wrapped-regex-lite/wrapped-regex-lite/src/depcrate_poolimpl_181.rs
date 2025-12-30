// Generated macro for impl_181 (impl)
macro_rules! Depcrate_poolimpl_181 {
() => {
// Module: crate::pool
// Provides: {"impl_181"}
// Dependencies: {}
impl < 'a , T : Send , F : Fn () -> T > core :: ops :: DerefMut for PoolGuard < 'a , T , F > { fn deref_mut (& mut self) -> & mut T { self . value . as_deref_mut () . unwrap () } }
};
}
