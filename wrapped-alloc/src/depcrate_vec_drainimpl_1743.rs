// Generated macro for impl_1743 (impl)
macro_rules! Depcrate_vec_drainimpl_1743 {
() => {
// Module: crate::vec::drain
// Provides: {"impl_1743"}
// Dependencies: {}
# [stable (feature = "drain" , since = "1.6.0")] impl < T , A : Allocator > Iterator for Drain < '_ , T , A > { type Item = T ; # [inline] fn next (& mut self) -> Option < T > { self . iter . next () . map (| elt | unsafe { ptr :: read (elt as * const _) }) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
