// Generated macro for scan_blank_line (function)
macro_rules! Depcrate_scannersscan_blank_line {
() => {
// Module: crate::scanners
// Provides: {"scan_blank_line"}
// Dependencies: {}
pub (crate) fn scan_blank_line (bytes : & [u8]) -> Option < usize > { let i = scan_whitespace_no_nl (bytes) ; scan_eol (& bytes [i ..]) . map (| n | i + n) }
};
}
