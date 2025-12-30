// Generated macro for impl_814 (impl)
macro_rules! Depcrate_util_poolimpl_814 {
() => {
// Module: crate::util::pool
// Provides: {"impl_814"}
// Dependencies: {}
impl < 'a , T : Send , F : Fn () -> T > core :: ops :: Deref for PoolGuard < 'a , T , F > { type Target = T ; # [inline] fn deref (& self) -> & T { self . 0 . value () } }
};
}
