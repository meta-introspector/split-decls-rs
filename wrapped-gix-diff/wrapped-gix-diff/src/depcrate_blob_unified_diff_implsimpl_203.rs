// Generated macro for impl_203 (impl)
macro_rules! Depcrate_blob_unified_diff_implsimpl_203 {
() => {
// Module: crate::blob::unified_diff::impls
// Provides: {"impl_203"}
// Dependencies: {}
# [doc = " An implementation that fails if the input isn't UTF-8."] impl < D > ConsumeHunk for ConsumeBinaryHunk < '_ , D > where D : ConsumeBinaryHunkDelegate , { type Out = D ; fn consume_hunk (& mut self , header : HunkHeader , lines : & [(DiffLineKind , & [u8])]) -> std :: io :: Result < () > { self . header_buf . clear () ; self . header_buf . write_fmt (format_args ! ("{header}{nl}" , nl = self . newline)) . map_err (std :: io :: Error :: other) ? ; let buf = & mut self . hunk_buf ; buf . clear () ; for & (line_type , content) in lines { buf . push (line_type . to_prefix () as u8) ; buf . extend_from_slice (content) ; if ! content . ends_with_str (self . newline) { buf . push_str (self . newline) ; } } self . delegate . consume_binary_hunk (header , & self . header_buf , buf) ? ; Ok (()) } fn finish (self) -> Self :: Out { self . delegate } }
};
}
