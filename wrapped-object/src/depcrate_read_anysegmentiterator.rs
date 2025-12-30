// Generated macro for SegmentIterator (struct)
macro_rules! Depcrate_read_anySegmentIterator {
() => {
// Module: crate::read::any
// Provides: {"SegmentIterator"}
// Dependencies: {}
# [doc = " An iterator for the loadable segments in a [`File`]."] # [derive (Debug)] pub struct SegmentIterator < 'data , 'file , R : ReadRef < 'data > = & 'data [u8] > { inner : SegmentIteratorInternal < 'data , 'file , R > , }
};
}
