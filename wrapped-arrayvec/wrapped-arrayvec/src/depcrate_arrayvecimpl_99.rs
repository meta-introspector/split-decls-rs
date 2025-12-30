// Generated macro for impl_99 (impl)
macro_rules! Depcrate_arrayvecimpl_99 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_99"}
// Dependencies: {}
impl < 'a , T : 'a , const CAP : usize > DoubleEndedIterator for Drain < 'a , T , CAP > { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . next_back () . map (| elt | unsafe { ptr :: read (elt as * const _) }) } }
};
}
