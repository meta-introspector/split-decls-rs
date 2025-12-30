// Generated macro for XcoffSection64 (type)
macro_rules! Depcrate_read_xcoff_sectionXcoffSection64 {
() => {
// Module: crate::read::xcoff::section
// Provides: {"XcoffSection64"}
// Dependencies: {}
# [doc = " A section in an [`XcoffFile64`](super::XcoffFile64)."] pub type XcoffSection64 < 'data , 'file , R = & 'data [u8] > = XcoffSection < 'data , 'file , xcoff :: FileHeader64 , R > ;
};
}
