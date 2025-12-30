// Generated macro for impl_765 (impl)
macro_rules! Depcrate_odbimpl_765 {
() => {
// Module: crate::odb
// Provides: {"impl_765"}
// Dependencies: {}
impl < 'a > Drop for OdbObject < 'a > { fn drop (& mut self) { unsafe { raw :: git_odb_object_free (self . raw) } } }
};
}
