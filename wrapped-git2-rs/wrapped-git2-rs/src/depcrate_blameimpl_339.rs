// Generated macro for impl_339 (impl)
macro_rules! Depcrate_blameimpl_339 {
() => {
// Module: crate::blame
// Provides: {"impl_339"}
// Dependencies: {}
impl < 'repo > Drop for Blame < 'repo > { fn drop (& mut self) { unsafe { raw :: git_blame_free (self . raw) } } }
};
}
