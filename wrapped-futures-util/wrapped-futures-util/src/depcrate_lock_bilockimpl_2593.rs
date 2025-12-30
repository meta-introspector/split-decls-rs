// Generated macro for impl_2593 (impl)
macro_rules! Depcrate_lock_bilockimpl_2593 {
() => {
// Module: crate::lock::bilock
// Provides: {"impl_2593"}
// Dependencies: {}
impl < T > Drop for Inner < T > { fn drop (& mut self) { assert ! (self . state . load (SeqCst) . is_null ()) ; } }
};
}
