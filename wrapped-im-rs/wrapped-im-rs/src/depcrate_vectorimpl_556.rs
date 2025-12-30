// Generated macro for impl_556 (impl)
macro_rules! Depcrate_vectorimpl_556 {
() => {
// Module: crate::vector
// Provides: {"impl_556"}
// Dependencies: {}
impl < A : Clone > DoubleEndedIterator for ConsumingIter < A > { # [doc = " Remove and return an element from the back of the iterator."] # [doc = ""] # [doc = " Time: O(1)*"] fn next_back (& mut self) -> Option < Self :: Item > { self . vector . pop_back () } }
};
}
