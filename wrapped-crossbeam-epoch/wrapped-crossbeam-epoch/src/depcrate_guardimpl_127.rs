// Generated macro for impl_127 (impl)
macro_rules! Depcrate_guardimpl_127 {
() => {
// Module: crate::guard
// Provides: {"impl_127"}
// Dependencies: {}
impl Drop for Guard { # [inline] fn drop (& mut self) { if let Some (local) = unsafe { self . local . as_ref () } { local . unpin () ; } } }
};
}
