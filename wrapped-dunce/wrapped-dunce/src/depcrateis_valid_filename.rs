// Generated macro for is_valid_filename (function)
macro_rules! Depcrateis_valid_filename {
() => {
// Module: crate
// Provides: {"is_valid_filename"}
// Dependencies: {}
# [cfg (any (windows , test))] fn is_valid_filename (file_name : & OsStr) -> bool { if file_name . len () > 255 && windows_char_len (file_name) > 255 { return false ; } let byte_str = if let Some (s) = file_name . to_str () { s . as_bytes () } else { return false ; } ; if byte_str . is_empty () { return false ; } if byte_str . iter () . any (| & c | matches ! (c , 0 ..= 31 | b'<' | b'>' | b':' | b'"' | b'/' | b'\\' | b'|' | b'?' | b'*')) { return false } if matches ! (byte_str . last () , Some (b' ' | b'.')) { return false ; } true }
};
}
