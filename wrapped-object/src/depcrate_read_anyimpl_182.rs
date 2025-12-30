// Generated macro for impl_182 (impl)
macro_rules! Depcrate_read_anyimpl_182 {
() => {
// Module: crate::read::any
// Provides: {"impl_182"}
// Dependencies: {}
impl < 'data , 'file , R : ReadRef < 'data > > Iterator for SegmentIterator < 'data , 'file , R > { type Item = Segment < 'data , 'file , R > ; fn next (& mut self) -> Option < Self :: Item > { next_inner ! (self . inner , SegmentIteratorInternal , SegmentInternal) . map (| inner | Segment { inner }) } }
};
}
