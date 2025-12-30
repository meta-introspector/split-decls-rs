// Generated macro for impl_98 (impl)
macro_rules! Depcrate_arrayvecimpl_98 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_98"}
// Dependencies: {}
impl < 'a , T : 'a , const CAP : usize > Iterator for Drain < 'a , T , CAP > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| elt | unsafe { ptr :: read (elt as * const _) }) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
