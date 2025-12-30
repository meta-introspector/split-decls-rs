// Generated macro for impl_936 (impl)
macro_rules! Depcrate_referenceimpl_936 {
() => {
// Module: crate::reference
// Provides: {"impl_936"}
// Dependencies: {}
impl < 'repo > Drop for Reference < 'repo > { fn drop (& mut self) { unsafe { raw :: git_reference_free (self . raw) } } }
};
}
