// Generated macro for read_file_header (function)
macro_rules! Depcrateread_file_header {
() => {
// Module: crate
// Provides: {"read_file_header"}
// Dependencies: {}
# [must_use] pub fn read_file_header (bytes : & [u8] , expected_magic : & [u8 ; 4] , diagnostic_file_path : Option < & Path > , stream_tag : & str ,) -> Result < u32 , Box < dyn Error + Send + Sync > > { assert_eq ! (FILE_HEADER_SIZE , 8) ; let diagnostic_file_path = diagnostic_file_path . unwrap_or (Path :: new ("<in-memory>")) ; if bytes . len () < FILE_HEADER_SIZE { let msg = format ! ("Error reading {} stream in file `{}`: Expected file to contain at least `{:?}` bytes but found `{:?}` bytes" , stream_tag , diagnostic_file_path . display () , FILE_HEADER_SIZE , bytes . len ()) ; return Err (From :: from (msg)) ; } let actual_magic = & bytes [0 .. 4] ; if actual_magic != expected_magic { let msg = format ! ("Error reading {} stream in file `{}`: Expected file magic `{:?}` but found `{:?}`" , stream_tag , diagnostic_file_path . display () , expected_magic , actual_magic) ; return Err (From :: from (msg)) ; } let file_format_version = u32 :: from_le_bytes (bytes [4 .. 8] . try_into () . unwrap ()) ; Ok (file_format_version) }
};
}
