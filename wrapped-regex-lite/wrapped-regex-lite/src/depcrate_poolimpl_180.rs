// Generated macro for impl_180 (impl)
macro_rules! Depcrate_poolimpl_180 {
() => {
// Module: crate::pool
// Provides: {"impl_180"}
// Dependencies: {}
impl < 'a , T : Send , F : Fn () -> T > core :: ops :: Deref for PoolGuard < 'a , T , F > { type Target = T ; fn deref (& self) -> & T { self . value . as_deref () . unwrap () } }
};
}
