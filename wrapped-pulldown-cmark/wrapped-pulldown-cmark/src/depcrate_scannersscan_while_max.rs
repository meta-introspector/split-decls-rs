// Generated macro for scan_while_max (function)
macro_rules! Depcrate_scannersscan_while_max {
() => {
// Module: crate::scanners
// Provides: {"scan_while_max"}
// Dependencies: {}
pub (crate) fn scan_while_max < F > (data : & [u8] , mut f : F , m : usize) -> usize where F : FnMut (u8) -> bool , { data . iter () . enumerate () . take_while (| (i , c) | * i < m && f (* * c)) . count () }
};
}
