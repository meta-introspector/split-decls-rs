// Generated macro for scan_blockquote_start (function)
macro_rules! Depcrate_scannersscan_blockquote_start {
() => {
// Module: crate::scanners
// Provides: {"scan_blockquote_start"}
// Dependencies: {}
pub (crate) fn scan_blockquote_start (data : & [u8]) -> Option < usize > { if data . first () . copied () == Some (b'>') { let space = if data . get (1) . copied () == Some (b' ') { 1 } else { 0 } ; Some (1 + space) } else { None } }
};
}
