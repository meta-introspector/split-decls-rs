// Generated macro for impl_1113 (impl)
macro_rules! Depcrate_revwalkimpl_1113 {
() => {
// Module: crate::revwalk
// Provides: {"impl_1113"}
// Dependencies: {}
impl < 'repo > Drop for Revwalk < 'repo > { fn drop (& mut self) { unsafe { raw :: git_revwalk_free (self . raw) } } }
};
}
