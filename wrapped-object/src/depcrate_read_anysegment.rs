// Generated macro for Segment (struct)
macro_rules! Depcrate_read_anySegment {
() => {
// Module: crate::read::any
// Provides: {"Segment"}
// Dependencies: {}
# [doc = " A loadable segment in a [`File`]."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectSegment`] trait implementation."] pub struct Segment < 'data , 'file , R : ReadRef < 'data > = & 'data [u8] > { inner : SegmentInternal < 'data , 'file , R > , }
};
}
