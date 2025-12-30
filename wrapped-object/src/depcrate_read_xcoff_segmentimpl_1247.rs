// Generated macro for impl_1247 (impl)
macro_rules! Depcrate_read_xcoff_segmentimpl_1247 {
() => {
// Module: crate::read::xcoff::segment
// Provides: {"impl_1247"}
// Dependencies: {}
impl < 'data , 'file , Xcoff , R > Iterator for XcoffSegmentIterator < 'data , 'file , Xcoff , R > where Xcoff : FileHeader , R : ReadRef < 'data > , { type Item = XcoffSegment < 'data , 'file , Xcoff , R > ; fn next (& mut self) -> Option < Self :: Item > { None } }
};
}
