// Generated macro for impl_82 (impl)
macro_rules! Depcrate_dbghelpimpl_82 {
() => {
// Module: crate::dbghelp
// Provides: {"impl_82"}
// Dependencies: {}
impl Drop for Init { fn drop (& mut self) { unsafe { let r = ReleaseMutex (self . lock) ; debug_assert ! (r != 0) ; } } }
};
}
