// Generated macro for impl_204 (impl)
macro_rules! Depcrate_blob_unified_diff_implsimpl_204 {
() => {
// Module: crate::blob::unified_diff::impls
// Provides: {"impl_204"}
// Dependencies: {}
# [doc = " An implementation that fails if the input isn't UTF-8."] impl ConsumeBinaryHunkDelegate for String { fn consume_binary_hunk (& mut self , _header : HunkHeader , header_str : & str , hunk : & [u8]) -> std :: io :: Result < () > { self . push_str (header_str) ; self . push_str (hunk . to_str () . map_err (std :: io :: Error :: other) ?) ; Ok (()) } }
};
}
