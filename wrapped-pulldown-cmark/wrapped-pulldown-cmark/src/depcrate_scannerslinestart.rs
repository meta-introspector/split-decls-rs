// Generated macro for LineStart (struct)
macro_rules! Depcrate_scannersLineStart {
() => {
// Module: crate::scanners
// Provides: {"LineStart"}
// Dependencies: {}
# [doc = " Analysis of the beginning of a line, including indentation and container"] # [doc = " markers."] # [derive (Clone)] pub (crate) struct LineStart < 'a > { bytes : & 'a [u8] , ix : usize , tab_start : usize , spaces_remaining : usize , min_hrule_offset : usize , }
};
}
