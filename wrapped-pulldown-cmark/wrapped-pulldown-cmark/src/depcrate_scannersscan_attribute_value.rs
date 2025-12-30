// Generated macro for scan_attribute_value (function)
macro_rules! Depcrate_scannersscan_attribute_value {
() => {
// Module: crate::scanners
// Provides: {"scan_attribute_value"}
// Dependencies: {}
# [doc = " Returns the index immediately following the attribute value on success."] fn scan_attribute_value (data : & [u8] , mut i : usize , newline_handler : Option < & dyn Fn (& [u8]) -> usize > , buffer : & mut Vec < u8 > , buffer_ix : & mut usize ,) -> Option < usize > { match * data . get (i) ? { b @ b'"' | b @ b'\'' => { i += 1 ; while i < data . len () { if data [i] == b { return Some (i + 1) ; } if let Some (eol_bytes) = scan_eol (& data [i ..]) { let handler = newline_handler ? ; i += eol_bytes ; let skipped_bytes = handler (& data [i ..]) ; if skipped_bytes > 0 { buffer . extend (& data [* buffer_ix .. i]) ; * buffer_ix = i + skipped_bytes ; } i += skipped_bytes ; } else { i += 1 ; } } return None ; } b' ' | b'=' | b'>' | b'<' | b'`' | b'\n' | b'\r' => { return None ; } _ => { i += scan_attr_value_chars (& data [i ..]) ; } } Some (i) }
};
}
