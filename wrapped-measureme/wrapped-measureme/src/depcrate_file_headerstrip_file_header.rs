// Generated macro for strip_file_header (function)
macro_rules! Depcrate_file_headerstrip_file_header {
() => {
// Module: crate::file_header
// Provides: {"strip_file_header"}
// Dependencies: {}
pub fn strip_file_header (data : & [u8]) -> & [u8] { & data [FILE_HEADER_SIZE ..] }
};
}
