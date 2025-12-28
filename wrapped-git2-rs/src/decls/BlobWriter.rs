macro_rules! deps {
    () => {
        Object!();
    };
}

macro_rules! BlobWriter {
    () => {
        deps!();
        # [doc = " A structure to represent a git writestream for blobs"] pub struct BlobWriter < 'repo > { raw : * mut raw :: git_writestream , need_cleanup : bool , _marker : marker :: PhantomData < Object < 'repo > > , }
    };
}

BlobWriter!()