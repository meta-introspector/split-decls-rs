// Generated macro for XcoffComdatSectionIterator32 (type)
macro_rules! Depcrate_read_xcoff_comdatXcoffComdatSectionIterator32 {
() => {
// Module: crate::read::xcoff::comdat
// Provides: {"XcoffComdatSectionIterator32"}
// Dependencies: {}
# [doc = " An iterator for the sections in a COMDAT section group in a [`XcoffFile32`](super::XcoffFile32)."] pub type XcoffComdatSectionIterator32 < 'data , 'file , R = & 'data [u8] > = XcoffComdatSectionIterator < 'data , 'file , xcoff :: FileHeader32 , R > ;
};
}
