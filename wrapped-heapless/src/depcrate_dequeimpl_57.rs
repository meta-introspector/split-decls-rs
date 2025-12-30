// Generated macro for impl_57 (impl)
macro_rules! Depcrate_dequeimpl_57 {
() => {
// Module: crate::deque
// Provides: {"impl_57"}
// Dependencies: {}
# [doc = " As with the standard library's `VecDeque`, items are added via `push_back`."] impl < T , S : VecStorage < T > + ? Sized > Extend < T > for DequeInner < T , S > { fn extend < I : IntoIterator < Item = T > > (& mut self , iter : I) { for item in iter { self . push_back (item) . ok () . unwrap () ; } } }
};
}
