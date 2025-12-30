// Generated macro for impl_197 (impl)
macro_rules! Depcrate_sync_wait_groupimpl_197 {
() => {
// Module: crate::sync::wait_group
// Provides: {"impl_197"}
// Dependencies: {}
impl Clone for WaitGroup { fn clone (& self) -> Self { let mut count = self . inner . count . lock () . unwrap () ; * count += 1 ; Self { inner : self . inner . clone () , } } }
};
}
