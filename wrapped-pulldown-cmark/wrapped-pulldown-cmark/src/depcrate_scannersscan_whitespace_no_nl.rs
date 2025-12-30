// Generated macro for scan_whitespace_no_nl (function)
macro_rules! Depcrate_scannersscan_whitespace_no_nl {
() => {
// Module: crate::scanners
// Provides: {"scan_whitespace_no_nl"}
// Dependencies: {}
pub (crate) fn scan_whitespace_no_nl (data : & [u8]) -> usize { scan_while (data , is_ascii_whitespace_no_nl) }
};
}
