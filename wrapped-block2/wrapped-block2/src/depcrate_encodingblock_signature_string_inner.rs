// Generated macro for block_signature_string_inner (function)
macro_rules! Depcrate_encodingblock_signature_string_inner {
() => {
// Module: crate::encoding
// Provides: {"block_signature_string_inner"}
// Dependencies: {}
# [allow (unused)] fn block_signature_string_inner (args : & [Encoding] , ret : & Encoding) -> CString { let arg_sizes = args . iter () . map (Encoding :: size) . map (Option :: unwrap_or_default) . collect :: < Vec < _ > > () ; let args_size = arg_sizes . iter () . sum :: < usize > () ; let mut off = mem :: size_of :: < * const () > () ; let mut res = ret . to_string () ; res . push_str (& (off + args_size) . to_string ()) ; res . push_str ("@?0") ; for (arg_enc , arg_size) in args . iter () . zip (arg_sizes) { res . push_str (& arg_enc . to_string ()) ; res . push_str (& off . to_string ()) ; off += arg_size ; } CString :: new (res) . unwrap () }
};
}
