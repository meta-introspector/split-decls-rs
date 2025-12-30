// Generated macro for XcoffSectionIterator64 (type)
macro_rules! Depcrate_read_xcoff_sectionXcoffSectionIterator64 {
() => {
// Module: crate::read::xcoff::section
// Provides: {"XcoffSectionIterator64"}
// Dependencies: {}
# [doc = " An iterator for the sections in an [`XcoffFile64`](super::XcoffFile64)."] pub type XcoffSectionIterator64 < 'data , 'file , R = & 'data [u8] > = XcoffSectionIterator < 'data , 'file , xcoff :: FileHeader64 , R > ;
};
}
