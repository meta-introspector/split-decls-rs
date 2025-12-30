// Generated macro for impl_359 (impl)
macro_rules! Depcrate_blobimpl_359 {
() => {
// Module: crate::blob
// Provides: {"impl_359"}
// Dependencies: {}
impl < 'repo > Drop for Blob < 'repo > { fn drop (& mut self) { unsafe { raw :: git_blob_free (self . raw) } } }
};
}
