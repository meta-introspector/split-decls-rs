// Generated macro for impl_1086 (impl)
macro_rules! Depcrate_repoimpl_1086 {
() => {
// Module: crate::repo
// Provides: {"impl_1086"}
// Dependencies: {}
impl Drop for Repository { fn drop (& mut self) { unsafe { raw :: git_repository_free (self . raw) } } }
};
}
