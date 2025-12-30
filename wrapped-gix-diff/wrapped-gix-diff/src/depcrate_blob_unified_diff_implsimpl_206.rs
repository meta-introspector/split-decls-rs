// Generated macro for impl_206 (impl)
macro_rules! Depcrate_blob_unified_diff_implsimpl_206 {
() => {
// Module: crate::blob::unified_diff::impls
// Provides: {"impl_206"}
// Dependencies: {}
# [doc = " An implementation that writes hunks into a hunman-readable byte buffer."] impl ConsumeBinaryHunkDelegate for BString { fn consume_binary_hunk (& mut self , _header : HunkHeader , header_str : & str , hunk : & [u8]) -> std :: io :: Result < () > { self . push_str (header_str) ; self . extend_from_slice (hunk) ; Ok (()) } }
};
}
