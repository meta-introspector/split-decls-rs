// Generated macro for scan_closing_code_fence (function)
macro_rules! Depcrate_scannersscan_closing_code_fence {
() => {
// Module: crate::scanners
// Provides: {"scan_closing_code_fence"}
// Dependencies: {}
pub (crate) fn scan_closing_code_fence (bytes : & [u8] , fence_char : u8 , n_fence_char : usize ,) -> Option < usize > { if bytes . is_empty () { return Some (0) ; } let mut i = 0 ; let num_fence_chars_found = scan_ch_repeat (& bytes [i ..] , fence_char) ; if num_fence_chars_found < n_fence_char { return None ; } i += num_fence_chars_found ; let num_trailing_spaces = scan_ch_repeat (& bytes [i ..] , b' ') ; i += num_trailing_spaces ; scan_eol (& bytes [i ..]) . map (| _ | i) }
};
}
