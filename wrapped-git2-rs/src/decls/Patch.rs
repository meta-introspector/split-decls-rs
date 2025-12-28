macro_rules! Patch {
    () => {
        # [doc = " A structure representing the text changes in a single diff delta."] # [doc = ""] # [doc = " This is an opaque structure."] pub struct Patch < 'buffers > { raw : * mut raw :: git_patch , buffers : PhantomData < & 'buffers () > , }
    };
}

Patch!()