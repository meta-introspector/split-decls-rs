// Generated macro for XcoffSectionIterator32 (type)
macro_rules! Depcrate_read_xcoff_sectionXcoffSectionIterator32 {
() => {
// Module: crate::read::xcoff::section
// Provides: {"XcoffSectionIterator32"}
// Dependencies: {}
# [doc = " An iterator for the sections in an [`XcoffFile32`](super::XcoffFile32)."] pub type XcoffSectionIterator32 < 'data , 'file , R = & 'data [u8] > = XcoffSectionIterator < 'data , 'file , xcoff :: FileHeader32 , R > ;
};
}
