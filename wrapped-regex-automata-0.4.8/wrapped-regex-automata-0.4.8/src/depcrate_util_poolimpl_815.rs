// Generated macro for impl_815 (impl)
macro_rules! Depcrate_util_poolimpl_815 {
() => {
// Module: crate::util::pool
// Provides: {"impl_815"}
// Dependencies: {}
impl < 'a , T : Send , F : Fn () -> T > core :: ops :: DerefMut for PoolGuard < 'a , T , F > { # [inline] fn deref_mut (& mut self) -> & mut T { self . 0 . value_mut () } }
};
}
