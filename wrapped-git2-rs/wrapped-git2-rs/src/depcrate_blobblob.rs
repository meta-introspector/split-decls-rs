// Generated macro for Blob (struct)
macro_rules! Depcrate_blobBlob {
() => {
// Module: crate::blob
// Provides: {"Blob"}
// Dependencies: {}
# [doc = " A structure to represent a git [blob][1]"] # [doc = ""] # [doc = " [1]: http://git-scm.com/book/en/Git-Internals-Git-Objects"] pub struct Blob < 'repo > { raw : * mut raw :: git_blob , _marker : marker :: PhantomData < Object < 'repo > > , }
};
}
