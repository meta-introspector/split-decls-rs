// Generated macro for impl_61 (impl)
macro_rules! Depcrate_dequeimpl_61 {
() => {
// Module: crate::deque
// Provides: {"impl_61"}
// Dependencies: {}
impl < T , const N : usize > DoubleEndedIterator for IntoIter < T , N > { fn next_back (& mut self) -> Option < Self :: Item > { self . deque . pop_back () } }
};
}
