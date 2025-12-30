// Generated macro for impl_864 (impl)
macro_rules! Depcrate_pathspecimpl_864 {
() => {
// Module: crate::pathspec
// Provides: {"impl_864"}
// Dependencies: {}
impl < 'ps > Drop for PathspecMatchList < 'ps > { fn drop (& mut self) { unsafe { raw :: git_pathspec_match_list_free (self . raw) } } }
};
}
