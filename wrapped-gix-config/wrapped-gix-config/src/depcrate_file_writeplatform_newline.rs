// Generated macro for platform_newline (function)
macro_rules! Depcrate_file_writeplatform_newline {
() => {
// Module: crate::file::write
// Provides: {"platform_newline"}
// Dependencies: {}
pub (crate) fn platform_newline () -> & 'static BStr { if cfg ! (windows) { "\r\n" } else { "\n" } . into () }
};
}
