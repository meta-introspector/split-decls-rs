// Generated macro for impl_550 (impl)
macro_rules! Depcrate_winmd_rowimpl_550 {
() => {
// Module: crate::winmd::row
// Provides: {"impl_550"}
// Dependencies: {}
impl < R : AsRow > Iterator for RowIterator < R > { type Item = R ; fn next (& mut self) -> Option < Self :: Item > { self . rows . next () . map (| row | R :: from_row (Row :: new (self . file , row))) } }
};
}
