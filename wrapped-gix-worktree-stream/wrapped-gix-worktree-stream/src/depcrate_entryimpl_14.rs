// Generated macro for impl_14 (impl)
macro_rules! Depcrate_entryimpl_14 {
() => {
// Module: crate::entry
// Provides: {"impl_14"}
// Dependencies: {}
impl Drop for Entry < '_ > { fn drop (& mut self) { if self . remaining == Some (0) { self . parent . path_buf = self . path_buf . take () ; } } }
};
}
