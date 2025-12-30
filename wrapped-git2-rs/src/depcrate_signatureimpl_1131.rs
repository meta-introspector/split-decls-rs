// Generated macro for impl_1131 (impl)
macro_rules! Depcrate_signatureimpl_1131 {
() => {
// Module: crate::signature
// Provides: {"impl_1131"}
// Dependencies: {}
impl < 'a > Drop for Signature < 'a > { fn drop (& mut self) { if self . owned { unsafe { raw :: git_signature_free (self . raw) } } } }
};
}
