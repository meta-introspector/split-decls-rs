// Generated macro for scan_while (function)
macro_rules! Depcrate_scannersscan_while {
() => {
// Module: crate::scanners
// Provides: {"scan_while"}
// Dependencies: {}
pub (crate) fn scan_while < F > (data : & [u8] , mut f : F) -> usize where F : FnMut (u8) -> bool , { data . iter () . take_while (| & & c | f (c)) . count () }
};
}
