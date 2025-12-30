// Generated macro for impl_23 (impl)
macro_rules! Depcrate_interruptimpl_23 {
() => {
// Module: crate::interrupt
// Provides: {"impl_23"}
// Dependencies: {}
impl < I > Iterator for Iter < '_ , I > where I : Iterator , { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { if self . should_interrupt . load (Ordering :: Relaxed) { return None ; } self . inner . next () } }
};
}
