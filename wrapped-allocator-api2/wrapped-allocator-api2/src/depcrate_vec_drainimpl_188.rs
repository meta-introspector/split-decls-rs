// Generated macro for impl_188 (impl)
macro_rules! Depcrate_vec_drainimpl_188 {
() => {
// Module: crate::vec::drain
// Provides: {"impl_188"}
// Dependencies: {}
impl < T , A : Allocator > Iterator for Drain < '_ , T , A > { type Item = T ; # [inline (always)] fn next (& mut self) -> Option < T > { self . iter . next () . map (| elt | unsafe { ptr :: read (elt as * const _) }) } # [inline (always)] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
