// Generated macro for impl_196 (impl)
macro_rules! Depcrate_sync_wait_groupimpl_196 {
() => {
// Module: crate::sync::wait_group
// Provides: {"impl_196"}
// Dependencies: {}
impl Drop for WaitGroup { fn drop (& mut self) { let mut count = self . inner . count . lock () . unwrap () ; * count -= 1 ; if * count == 0 { self . inner . cvar . notify_all () ; } } }
};
}
