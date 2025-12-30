// Generated macro for impl_555 (impl)
macro_rules! Depcrate_vectorimpl_555 {
() => {
// Module: crate::vector
// Provides: {"impl_555"}
// Dependencies: {}
impl < A : Clone > Iterator for ConsumingIter < A > { type Item = A ; # [doc = " Advance the iterator and return the next value."] # [doc = ""] # [doc = " Time: O(1)*"] fn next (& mut self) -> Option < Self :: Item > { self . vector . pop_front () } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . vector . len () ; (len , Some (len)) } }
};
}
