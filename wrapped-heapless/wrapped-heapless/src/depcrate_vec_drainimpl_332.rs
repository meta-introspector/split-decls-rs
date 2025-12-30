// Generated macro for impl_332 (impl)
macro_rules! Depcrate_vec_drainimpl_332 {
() => {
// Module: crate::vec::drain
// Provides: {"impl_332"}
// Dependencies: {}
impl < T , LenT : LenType > Iterator for Drain < '_ , T , LenT > { type Item = T ; # [inline] fn next (& mut self) -> Option < T > { self . iter . next () . map (| elt | unsafe { ptr :: read (core :: ptr :: from_ref (elt)) }) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
