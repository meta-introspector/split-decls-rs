// Generated macro for BlobWriter (struct)
macro_rules! Depcrate_blobBlobWriter {
() => {
// Module: crate::blob
// Provides: {"BlobWriter"}
// Dependencies: {}
# [doc = " A structure to represent a git writestream for blobs"] pub struct BlobWriter < 'repo > { raw : * mut raw :: git_writestream , need_cleanup : bool , _marker : marker :: PhantomData < Object < 'repo > > , }
};
}
