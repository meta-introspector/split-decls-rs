// Generated macro for impl_527 (impl)
macro_rules! Depcrate_diffimpl_527 {
() => {
// Module: crate::diff
// Provides: {"impl_527"}
// Dependencies: {}
impl < 'repo > Drop for Diff < 'repo > { fn drop (& mut self) { unsafe { raw :: git_diff_free (self . raw) } } }
};
}
