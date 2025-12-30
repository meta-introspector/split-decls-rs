// Generated macro for impl_1744 (impl)
macro_rules! Depcrate_vec_drainimpl_1744 {
() => {
// Module: crate::vec::drain
// Provides: {"impl_1744"}
// Dependencies: {}
# [stable (feature = "drain" , since = "1.6.0")] impl < T , A : Allocator > DoubleEndedIterator for Drain < '_ , T , A > { # [inline] fn next_back (& mut self) -> Option < T > { self . iter . next_back () . map (| elt | unsafe { ptr :: read (elt as * const _) }) } }
};
}
