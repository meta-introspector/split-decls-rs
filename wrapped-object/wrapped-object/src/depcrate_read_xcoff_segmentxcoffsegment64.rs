// Generated macro for XcoffSegment64 (type)
macro_rules! Depcrate_read_xcoff_segmentXcoffSegment64 {
() => {
// Module: crate::read::xcoff::segment
// Provides: {"XcoffSegment64"}
// Dependencies: {}
# [doc = " A segment in an [`XcoffFile64`](super::XcoffFile64)."] pub type XcoffSegment64 < 'data , 'file , R = & 'data [u8] > = XcoffSegment < 'data , 'file , xcoff :: FileHeader64 , R > ;
};
}
