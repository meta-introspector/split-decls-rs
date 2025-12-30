// Generated macro for impl_271 (impl)
macro_rules! Depcrate_ord_setimpl_271 {
() => {
// Module: crate::ord::set
// Provides: {"impl_271"}
// Dependencies: {}
impl < 'a , A > Iterator for RangedIter < 'a , A > where A : 'a + Ord , { type Item = & 'a A ; # [doc = " Advance the iterator and return the next value."] # [doc = ""] # [doc = " Time: O(1)*"] fn next (& mut self) -> Option < Self :: Item > { self . it . next () . map (Deref :: deref) } fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
};
}
