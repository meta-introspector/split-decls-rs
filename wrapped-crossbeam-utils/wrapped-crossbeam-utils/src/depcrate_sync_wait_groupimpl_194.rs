// Generated macro for impl_194 (impl)
macro_rules! Depcrate_sync_wait_groupimpl_194 {
() => {
// Module: crate::sync::wait_group
// Provides: {"impl_194"}
// Dependencies: {}
impl Default for WaitGroup { fn default () -> Self { Self { inner : Arc :: new (Inner { cvar : Condvar :: new () , count : Mutex :: new (1) , }) , } } }
};
}
