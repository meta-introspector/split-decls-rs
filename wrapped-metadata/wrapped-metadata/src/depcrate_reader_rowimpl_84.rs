// Generated macro for impl_84 (impl)
macro_rules! Depcrate_reader_rowimpl_84 {
() => {
// Module: crate::reader::row
// Provides: {"impl_84"}
// Dependencies: {}
impl < 'a , R : AsRow < 'a > > Iterator for RowIterator < 'a , R > { type Item = R ; fn next (& mut self) -> Option < Self :: Item > { self . rows . next () . map (| row | R :: from_row (Row :: new (self . index , self . file , row))) } }
};
}
