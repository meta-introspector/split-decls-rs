// Generated macro for impl_274 (impl)
macro_rules! Depcrate_ord_setimpl_274 {
() => {
// Module: crate::ord::set
// Provides: {"impl_274"}
// Dependencies: {}
impl < A > Iterator for ConsumingIter < A > where A : Ord + Clone , { type Item = A ; # [doc = " Advance the iterator and return the next value."] # [doc = ""] # [doc = " Time: O(1)*"] fn next (& mut self) -> Option < Self :: Item > { self . it . next () . map (| v | v . 0) } }
};
}
