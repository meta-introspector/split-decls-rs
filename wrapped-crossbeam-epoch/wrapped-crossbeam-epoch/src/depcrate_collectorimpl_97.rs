// Generated macro for impl_97 (impl)
macro_rules! Depcrate_collectorimpl_97 {
() => {
// Module: crate::collector
// Provides: {"impl_97"}
// Dependencies: {}
impl Drop for LocalHandle { # [inline] fn drop (& mut self) { unsafe { Local :: release_handle (& * self . local) ; } } }
};
}
