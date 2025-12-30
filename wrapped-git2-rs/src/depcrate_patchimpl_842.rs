// Generated macro for impl_842 (impl)
macro_rules! Depcrate_patchimpl_842 {
() => {
// Module: crate::patch
// Provides: {"impl_842"}
// Dependencies: {}
impl < 'buffers > Drop for Patch < 'buffers > { fn drop (& mut self) { unsafe { raw :: git_patch_free (self . raw) } } }
};
}
