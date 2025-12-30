// Generated macro for scan_closing_metadata_block (function)
macro_rules! Depcrate_scannersscan_closing_metadata_block {
() => {
// Module: crate::scanners
// Provides: {"scan_closing_metadata_block"}
// Dependencies: {}
pub (crate) fn scan_closing_metadata_block (bytes : & [u8] , fence_char : u8) -> Option < usize > { let mut i = 0 ; let mut num_fence_chars_found = scan_ch_repeat (& bytes [i ..] , fence_char) ; if num_fence_chars_found != 3 { if fence_char == b'-' { num_fence_chars_found = scan_ch_repeat (& bytes [i ..] , b'.') ; if num_fence_chars_found != 3 { return None ; } } else { return None ; } } i += num_fence_chars_found ; let num_trailing_spaces = scan_ch_repeat (& bytes [i ..] , b' ') ; i += num_trailing_spaces ; scan_eol (& bytes [i ..]) . map (| _ | i) }
};
}
