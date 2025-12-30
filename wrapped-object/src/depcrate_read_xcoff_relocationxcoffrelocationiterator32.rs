// Generated macro for XcoffRelocationIterator32 (type)
macro_rules! Depcrate_read_xcoff_relocationXcoffRelocationIterator32 {
() => {
// Module: crate::read::xcoff::relocation
// Provides: {"XcoffRelocationIterator32"}
// Dependencies: {}
# [doc = " An iterator for the relocations in an [`XcoffSection32`](super::XcoffSection32)."] pub type XcoffRelocationIterator32 < 'data , 'file , R = & 'data [u8] > = XcoffRelocationIterator < 'data , 'file , xcoff :: FileHeader32 , R > ;
};
}
