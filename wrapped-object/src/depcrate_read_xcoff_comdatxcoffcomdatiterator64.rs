// Generated macro for XcoffComdatIterator64 (type)
macro_rules! Depcrate_read_xcoff_comdatXcoffComdatIterator64 {
() => {
// Module: crate::read::xcoff::comdat
// Provides: {"XcoffComdatIterator64"}
// Dependencies: {}
# [doc = " An iterator for the COMDAT section groups in a [`XcoffFile64`](super::XcoffFile64)."] pub type XcoffComdatIterator64 < 'data , 'file , R = & 'data [u8] > = XcoffComdatIterator < 'data , 'file , xcoff :: FileHeader64 , R > ;
};
}
