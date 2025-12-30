// Generated macro for XcoffSegmentIterator64 (type)
macro_rules! Depcrate_read_xcoff_segmentXcoffSegmentIterator64 {
() => {
// Module: crate::read::xcoff::segment
// Provides: {"XcoffSegmentIterator64"}
// Dependencies: {}
# [doc = " An iterator for the segments in an [`XcoffFile64`](super::XcoffFile64)."] pub type XcoffSegmentIterator64 < 'data , 'file , R = & 'data [u8] > = XcoffSegmentIterator < 'data , 'file , xcoff :: FileHeader64 , R > ;
};
}
