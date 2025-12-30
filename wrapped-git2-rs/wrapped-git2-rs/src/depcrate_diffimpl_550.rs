// Generated macro for impl_550 (impl)
macro_rules! Depcrate_diffimpl_550 {
() => {
// Module: crate::diff
// Provides: {"impl_550"}
// Dependencies: {}
impl Drop for DiffStats { fn drop (& mut self) { unsafe { raw :: git_diff_stats_free (self . raw) } } }
};
}
