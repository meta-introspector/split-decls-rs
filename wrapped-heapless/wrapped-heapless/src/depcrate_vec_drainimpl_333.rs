// Generated macro for impl_333 (impl)
macro_rules! Depcrate_vec_drainimpl_333 {
() => {
// Module: crate::vec::drain
// Provides: {"impl_333"}
// Dependencies: {}
impl < T , LenT : LenType > DoubleEndedIterator for Drain < '_ , T , LenT > { # [inline] fn next_back (& mut self) -> Option < T > { self . iter . next_back () . map (| elt | unsafe { ptr :: read (core :: ptr :: from_ref (elt)) }) } }
};
}
