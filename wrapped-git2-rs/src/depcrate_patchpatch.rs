// Generated macro for Patch (struct)
macro_rules! Depcrate_patchPatch {
() => {
// Module: crate::patch
// Provides: {"Patch"}
// Dependencies: {}
# [doc = " A structure representing the text changes in a single diff delta."] # [doc = ""] # [doc = " This is an opaque structure."] pub struct Patch < 'buffers > { raw : * mut raw :: git_patch , buffers : PhantomData < & 'buffers () > , }
};
}
