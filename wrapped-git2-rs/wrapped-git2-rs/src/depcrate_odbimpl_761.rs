// Generated macro for impl_761 (impl)
macro_rules! Depcrate_odbimpl_761 {
() => {
// Module: crate::odb
// Provides: {"impl_761"}
// Dependencies: {}
impl < 'repo > Drop for Odb < 'repo > { fn drop (& mut self) { unsafe { raw :: git_odb_free (self . raw) } } }
};
}
