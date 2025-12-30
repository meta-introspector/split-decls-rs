// Generated macro for scan_eol (function)
macro_rules! Depcrate_scannersscan_eol {
() => {
// Module: crate::scanners
// Provides: {"scan_eol"}
// Dependencies: {}
pub (crate) fn scan_eol (bytes : & [u8]) -> Option < usize > { match bytes { & [] => Some (0) , & [b'\n' , ..] => Some (1) , & [b'\r' , b'\n' , ..] => Some (2) , & [b'\r' , ..] => Some (1) , _ => None , } }
};
}
