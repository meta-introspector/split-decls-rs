// Generated macro for impl_861 (impl)
macro_rules! Depcrate_pathspecimpl_861 {
() => {
// Module: crate::pathspec
// Provides: {"impl_861"}
// Dependencies: {}
impl Drop for Pathspec { fn drop (& mut self) { unsafe { raw :: git_pathspec_free (self . raw) } } }
};
}
