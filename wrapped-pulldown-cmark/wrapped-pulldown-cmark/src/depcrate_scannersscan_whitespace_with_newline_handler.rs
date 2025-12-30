// Generated macro for scan_whitespace_with_newline_handler (function)
macro_rules! Depcrate_scannersscan_whitespace_with_newline_handler {
() => {
// Module: crate::scanners
// Provides: {"scan_whitespace_with_newline_handler"}
// Dependencies: {}
# [doc = " Scans whitespace and possibly newlines according to the"] # [doc = " behavior defined by the newline handler. When bytes are skipped,"] # [doc = " all preceding non-skipped bytes are pushed to the buffer."] fn scan_whitespace_with_newline_handler (data : & [u8] , mut i : usize , newline_handler : Option < & dyn Fn (& [u8]) -> usize > , buffer : & mut Vec < u8 > , buffer_ix : & mut usize ,) -> Option < usize > { while i < data . len () { if ! is_ascii_whitespace (data [i]) { return Some (i) ; } if let Some (eol_bytes) = scan_eol (& data [i ..]) { let handler = newline_handler ? ; i += eol_bytes ; let skipped_bytes = handler (& data [i ..]) ; if skipped_bytes > 0 { buffer . extend (& data [* buffer_ix .. i]) ; * buffer_ix = i + skipped_bytes ; } i += skipped_bytes ; } else { i += 1 ; } } Some (i) }
};
}
