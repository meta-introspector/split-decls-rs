// Generated macro for impl_60 (impl)
macro_rules! Depcrate_dequeimpl_60 {
() => {
// Module: crate::deque
// Provides: {"impl_60"}
// Dependencies: {}
impl < T , const N : usize > Iterator for IntoIter < T , N > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { self . deque . pop_front () } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . len () ; (len , Some (len)) } }
};
}
