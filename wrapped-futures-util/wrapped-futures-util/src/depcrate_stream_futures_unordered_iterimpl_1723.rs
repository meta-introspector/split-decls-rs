// Generated macro for impl_1723 (impl)
macro_rules! Depcrate_stream_futures_unordered_iterimpl_1723 {
() => {
// Module: crate::stream::futures_unordered::iter
// Provides: {"impl_1723"}
// Dependencies: {}
impl < Fut : Unpin > Iterator for IntoIter < Fut > { type Item = Fut ; fn next (& mut self) -> Option < Self :: Item > { let task = self . inner . head_all . get_mut () ; if (* task) . is_null () { return None ; } unsafe { let future = (* (* * task) . future . get ()) . take () . unwrap () ; let next = (* * task) . next_all . load (Relaxed) ; * task = next ; if ! task . is_null () { * (* * task) . prev_all . get () = ptr :: null_mut () ; } self . len -= 1 ; Some (future) } } fn size_hint (& self) -> (usize , Option < usize >) { (self . len , Some (self . len)) } }
};
}
