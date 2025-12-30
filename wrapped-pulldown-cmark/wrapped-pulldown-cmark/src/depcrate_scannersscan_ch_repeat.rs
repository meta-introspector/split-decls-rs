// Generated macro for scan_ch_repeat (function)
macro_rules! Depcrate_scannersscan_ch_repeat {
() => {
// Module: crate::scanners
// Provides: {"scan_ch_repeat"}
// Dependencies: {}
pub (crate) fn scan_ch_repeat (data : & [u8] , c : u8) -> usize { scan_while (data , | x | x == c) }
};
}
