// Generated macro for impl_189 (impl)
macro_rules! Depcrate_vec_drainimpl_189 {
() => {
// Module: crate::vec::drain
// Provides: {"impl_189"}
// Dependencies: {}
impl < T , A : Allocator > DoubleEndedIterator for Drain < '_ , T , A > { # [inline (always)] fn next_back (& mut self) -> Option < T > { self . iter . next_back () . map (| elt | unsafe { ptr :: read (elt as * const _) }) } }
};
}
