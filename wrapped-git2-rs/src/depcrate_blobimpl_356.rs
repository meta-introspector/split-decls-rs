// Generated macro for impl_356 (impl)
macro_rules! Depcrate_blobimpl_356 {
() => {
// Module: crate::blob
// Provides: {"impl_356"}
// Dependencies: {}
impl < 'repo > Binding for Blob < 'repo > { type Raw = * mut raw :: git_blob ; unsafe fn from_raw (raw : * mut raw :: git_blob) -> Blob < 'repo > { Blob { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_blob { self . raw } }
};
}
