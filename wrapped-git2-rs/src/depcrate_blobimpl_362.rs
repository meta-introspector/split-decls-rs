// Generated macro for impl_362 (impl)
macro_rules! Depcrate_blobimpl_362 {
() => {
// Module: crate::blob
// Provides: {"impl_362"}
// Dependencies: {}
impl < 'repo > Binding for BlobWriter < 'repo > { type Raw = * mut raw :: git_writestream ; unsafe fn from_raw (raw : * mut raw :: git_writestream) -> BlobWriter < 'repo > { BlobWriter { raw , need_cleanup : true , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_writestream { self . raw } }
};
}
