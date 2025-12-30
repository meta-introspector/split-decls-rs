// Generated macro for XcoffComdatSectionIterator64 (type)
macro_rules! Depcrate_read_xcoff_comdatXcoffComdatSectionIterator64 {
() => {
// Module: crate::read::xcoff::comdat
// Provides: {"XcoffComdatSectionIterator64"}
// Dependencies: {}
# [doc = " An iterator for the sections in a COMDAT section group in a [`XcoffFile64`](super::XcoffFile64)."] pub type XcoffComdatSectionIterator64 < 'data , 'file , R = & 'data [u8] > = XcoffComdatSectionIterator < 'data , 'file , xcoff :: FileHeader64 , R > ;
};
}
