// Generated macro for XcoffSegmentIterator32 (type)
macro_rules! Depcrate_read_xcoff_segmentXcoffSegmentIterator32 {
() => {
// Module: crate::read::xcoff::segment
// Provides: {"XcoffSegmentIterator32"}
// Dependencies: {}
# [doc = " An iterator for the segments in an [`XcoffFile32`](super::XcoffFile32)."] pub type XcoffSegmentIterator32 < 'data , 'file , R = & 'data [u8] > = XcoffSegmentIterator < 'data , 'file , xcoff :: FileHeader32 , R > ;
};
}
