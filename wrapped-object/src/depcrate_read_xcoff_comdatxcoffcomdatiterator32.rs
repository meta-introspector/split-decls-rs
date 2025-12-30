// Generated macro for XcoffComdatIterator32 (type)
macro_rules! Depcrate_read_xcoff_comdatXcoffComdatIterator32 {
() => {
// Module: crate::read::xcoff::comdat
// Provides: {"XcoffComdatIterator32"}
// Dependencies: {}
# [doc = " An iterator for the COMDAT section groups in a [`XcoffFile32`](super::XcoffFile32)."] pub type XcoffComdatIterator32 < 'data , 'file , R = & 'data [u8] > = XcoffComdatIterator < 'data , 'file , xcoff :: FileHeader32 , R > ;
};
}
