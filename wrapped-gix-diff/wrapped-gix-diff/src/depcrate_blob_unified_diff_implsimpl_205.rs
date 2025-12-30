// Generated macro for impl_205 (impl)
macro_rules! Depcrate_blob_unified_diff_implsimpl_205 {
() => {
// Module: crate::blob::unified_diff::impls
// Provides: {"impl_205"}
// Dependencies: {}
# [doc = " An implementation that writes hunks into a byte buffer."] impl ConsumeBinaryHunkDelegate for Vec < u8 > { fn consume_binary_hunk (& mut self , _header : HunkHeader , header_str : & str , hunk : & [u8]) -> std :: io :: Result < () > { self . push_str (header_str) ; self . extend_from_slice (hunk) ; Ok (()) } }
};
}
