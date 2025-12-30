// Generated macro for impl_363 (impl)
macro_rules! Depcrate_blobimpl_363 {
() => {
// Module: crate::blob
// Provides: {"impl_363"}
// Dependencies: {}
impl < 'repo > Drop for BlobWriter < 'repo > { fn drop (& mut self) { if self . need_cleanup { unsafe { if let Some (f) = (* self . raw) . free { f (self . raw) } } } } }
};
}
