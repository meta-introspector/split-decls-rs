// Generated macro for impl_940 (impl)
macro_rules! Depcrate_referenceimpl_940 {
() => {
// Module: crate::reference
// Provides: {"impl_940"}
// Dependencies: {}
impl < 'repo > Drop for References < 'repo > { fn drop (& mut self) { unsafe { raw :: git_reference_iterator_free (self . raw) } } }
};
}
