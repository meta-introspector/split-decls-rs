// Generated macro for scan_ch (function)
macro_rules! Depcrate_scannersscan_ch {
() => {
// Module: crate::scanners
// Provides: {"scan_ch"}
// Dependencies: {}
pub (crate) fn scan_ch (data : & [u8] , c : u8) -> usize { if ! data . is_empty () && data [0] == c { 1 } else { 0 } }
};
}
