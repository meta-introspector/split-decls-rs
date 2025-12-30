// Generated macro for scan_html_type_7 (function)
macro_rules! Depcrate_scannersscan_html_type_7 {
() => {
// Module: crate::scanners
// Provides: {"scan_html_type_7"}
// Dependencies: {}
# [doc = " Assumes that `data` starts with `<`."] # [doc = " Returns the index into data directly after the html tag on success."] pub (crate) fn scan_html_type_7 (data : & [u8]) -> Option < usize > { let (_span , i) = scan_html_block_inner (data , None) ? ; scan_blank_line (& data [i ..]) ? ; Some (i) }
};
}
