// Generated macro for impl_1196 (impl)
macro_rules! Depcrate_submoduleimpl_1196 {
() => {
// Module: crate::submodule
// Provides: {"impl_1196"}
// Dependencies: {}
impl < 'repo > Drop for Submodule < 'repo > { fn drop (& mut self) { unsafe { raw :: git_submodule_free (self . raw) } } }
};
}
