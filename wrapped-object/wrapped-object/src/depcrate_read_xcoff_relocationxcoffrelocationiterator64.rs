// Generated macro for XcoffRelocationIterator64 (type)
macro_rules! Depcrate_read_xcoff_relocationXcoffRelocationIterator64 {
() => {
// Module: crate::read::xcoff::relocation
// Provides: {"XcoffRelocationIterator64"}
// Dependencies: {}
# [doc = " An iterator for the relocations in an [`XcoffSection64`](super::XcoffSection64)."] pub type XcoffRelocationIterator64 < 'data , 'file , R = & 'data [u8] > = XcoffRelocationIterator < 'data , 'file , xcoff :: FileHeader64 , R > ;
};
}
