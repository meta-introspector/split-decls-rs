// Generated macro for scan_whitespace_with_newline_handler_without_buffer (function)
macro_rules! Depcrate_scannersscan_whitespace_with_newline_handler_without_buffer {
() => {
// Module: crate::scanners
// Provides: {"scan_whitespace_with_newline_handler_without_buffer"}
// Dependencies: {}
# [doc = " Scans whitespace and possible newlines according to the behavior defined"] # [doc = " by the newline handler."] # [doc = ""] # [doc = " Unlike [`scan_whitespace_with_newline_handler`], this function doesn't"] # [doc = " copy skipped data into a buffer. Typically, if this function"] # [doc = " returns `Some`, a call to `scan_whitespace_with_newline_handler` will"] # [doc = " soon follow."] fn scan_whitespace_with_newline_handler_without_buffer (data : & [u8] , mut i : usize , newline_handler : Option < & dyn Fn (& [u8]) -> usize > ,) -> Option < usize > { while i < data . len () { if ! is_ascii_whitespace (data [i]) { return Some (i) ; } if let Some (eol_bytes) = scan_eol (& data [i ..]) { let handler = newline_handler ? ; i += eol_bytes ; let skipped_bytes = handler (& data [i ..]) ; i += skipped_bytes ; } else { i += 1 ; } } Some (i) }
};
}
