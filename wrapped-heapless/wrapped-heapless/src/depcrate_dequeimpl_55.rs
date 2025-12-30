// Generated macro for impl_55 (impl)
macro_rules! Depcrate_dequeimpl_55 {
() => {
// Module: crate::deque
// Provides: {"impl_55"}
// Dependencies: {}
impl < T , S : VecStorage < T > + ? Sized > Drop for DequeInner < T , S > { fn drop (& mut self) { unsafe { self . drop_contents () } } }
};
}
