// Generated macro for XcoffSegment32 (type)
macro_rules! Depcrate_read_xcoff_segmentXcoffSegment32 {
() => {
// Module: crate::read::xcoff::segment
// Provides: {"XcoffSegment32"}
// Dependencies: {}
# [doc = " A segment in an [`XcoffFile32`](super::XcoffFile32)."] pub type XcoffSegment32 < 'data , 'file , R = & 'data [u8] > = XcoffSegment < 'data , 'file , xcoff :: FileHeader32 , R > ;
};
}
