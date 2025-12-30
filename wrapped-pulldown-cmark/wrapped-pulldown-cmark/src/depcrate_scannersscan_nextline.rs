// Generated macro for scan_nextline (function)
macro_rules! Depcrate_scannersscan_nextline {
() => {
// Module: crate::scanners
// Provides: {"scan_nextline"}
// Dependencies: {}
pub (crate) fn scan_nextline (bytes : & [u8]) -> usize { memchr (b'\n' , bytes) . map_or (bytes . len () , | x | x + 1) }
};
}
