// Generated macro for detect_line_ending_or_nl (function)
macro_rules! Depcrate_blob_builtin_driver_text_utilsdetect_line_ending_or_nl {
() => {
// Module: crate::blob::builtin_driver::text::utils
// Provides: {"detect_line_ending_or_nl"}
// Dependencies: {}
pub fn detect_line_ending_or_nl (hunks : & [Hunk] , input : & mut InternedInput < & [u8] > , current_tokens : & [Token] ,) -> & 'static BStr { detect_line_ending (hunks , input , current_tokens) . unwrap_or (b"\n" . into ()) }
};
}
