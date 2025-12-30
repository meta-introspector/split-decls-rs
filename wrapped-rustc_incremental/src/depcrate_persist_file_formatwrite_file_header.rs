// Generated macro for write_file_header (function)
macro_rules! Depcrate_persist_file_formatwrite_file_header {
() => {
// Module: crate::persist::file_format
// Provides: {"write_file_header"}
// Dependencies: {}
pub (crate) fn write_file_header (stream : & mut FileEncoder , sess : & Session) { stream . emit_raw_bytes (FILE_MAGIC) ; stream . emit_raw_bytes (& [(HEADER_FORMAT_VERSION >> 0) as u8 , (HEADER_FORMAT_VERSION >> 8) as u8]) ; let rustc_version = rustc_version (sess . is_nightly_build () , sess . cfg_version) ; assert_eq ! (rustc_version . len () , (rustc_version . len () as u8) as usize) ; stream . emit_raw_bytes (& [rustc_version . len () as u8]) ; stream . emit_raw_bytes (rustc_version . as_bytes ()) ; }
};
}
