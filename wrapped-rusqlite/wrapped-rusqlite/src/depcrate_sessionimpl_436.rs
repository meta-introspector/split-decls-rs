// Generated macro for impl_436 (impl)
macro_rules! Depcrate_sessionimpl_436 {
() => {
// Module: crate::session
// Provides: {"impl_436"}
// Dependencies: {}
impl Drop for Session < '_ > { # [inline] fn drop (& mut self) { if self . filter . is_some () { self . table_filter (None :: < fn (& str) -> bool >) ; } unsafe { ffi :: sqlite3session_delete (self . s) } ; } }
};
}
