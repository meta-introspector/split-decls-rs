// Generated macro for XcoffSection32 (type)
macro_rules! Depcrate_read_xcoff_sectionXcoffSection32 {
() => {
// Module: crate::read::xcoff::section
// Provides: {"XcoffSection32"}
// Dependencies: {}
# [doc = " A section in an [`XcoffFile32`](super::XcoffFile32)."] pub type XcoffSection32 < 'data , 'file , R = & 'data [u8] > = XcoffSection < 'data , 'file , xcoff :: FileHeader32 , R > ;
};
}
