// Generated macro for scan_rev_while (function)
macro_rules! Depcrate_scannersscan_rev_while {
() => {
// Module: crate::scanners
// Provides: {"scan_rev_while"}
// Dependencies: {}
pub (crate) fn scan_rev_while < F > (data : & [u8] , mut f : F) -> usize where F : FnMut (u8) -> bool , { data . iter () . rev () . take_while (| & & c | f (c)) . count () }
};
}
