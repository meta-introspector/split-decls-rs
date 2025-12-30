// Generated macro for write_file_header (function)
macro_rules! Depcrate_file_headerwrite_file_header {
() => {
// Module: crate::file_header
// Provides: {"write_file_header"}
// Dependencies: {}
pub fn write_file_header (s : & mut dyn std :: io :: Write , file_magic : & [u8 ; 4] ,) -> Result < () , Box < dyn Error + Send + Sync > > { assert_eq ! (FILE_HEADER_SIZE , 8) ; s . write_all (file_magic) . map_err (Box :: new) ? ; s . write_all (& CURRENT_FILE_FORMAT_VERSION . to_le_bytes ()) . map_err (Box :: new) ? ; Ok (()) }
};
}
