// Generated macro for impl_447 (impl)
macro_rules! Depcrate_sessionimpl_447 {
() => {
// Module: crate::session
// Provides: {"impl_447"}
// Dependencies: {}
impl Drop for ChangesetIter < '_ > { # [inline] fn drop (& mut self) { unsafe { ffi :: sqlite3changeset_finalize (self . it) ; } } }
};
}
