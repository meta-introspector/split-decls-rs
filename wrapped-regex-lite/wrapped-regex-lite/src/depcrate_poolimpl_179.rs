// Generated macro for impl_179 (impl)
macro_rules! Depcrate_poolimpl_179 {
() => {
// Module: crate::pool
// Provides: {"impl_179"}
// Dependencies: {}
impl < 'a , T : Send , F : Fn () -> T > Drop for PoolGuard < 'a , T , F > { fn drop (& mut self) { if let Some (value) = self . value . take () { self . pool . put_value (value) ; } } }
};
}
