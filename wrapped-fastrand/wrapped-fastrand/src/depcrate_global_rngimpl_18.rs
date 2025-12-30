// Generated macro for impl_18 (impl)
macro_rules! Depcrate_global_rngimpl_18 {
() => {
// Module: crate::global_rng
// Provides: {"impl_18"}
// Dependencies: {}
impl Drop for RestoreOnDrop < '_ > { fn drop (& mut self) { self . rng . set (Rng (self . current . 0)) ; } }
};
}
