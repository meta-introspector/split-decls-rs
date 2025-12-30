// Generated macro for impl_818 (impl)
macro_rules! Depcrate_util_poolimpl_818 {
() => {
// Module: crate::util::pool
// Provides: {"impl_818"}
// Dependencies: {}
impl < 'a , T : Send , F : Fn () -> T > core :: ops :: DerefMut for PoolGuard < 'a , T , F > { # [inline] fn deref_mut (& mut self) -> & mut T { self . 0 . value_mut () } }
};
}
