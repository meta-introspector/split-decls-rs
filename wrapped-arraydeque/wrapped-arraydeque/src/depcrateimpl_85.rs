// Generated macro for impl_85 (impl)
macro_rules! Depcrateimpl_85 {
() => {
// Module: crate
// Provides: {"impl_85"}
// Dependencies: {}
impl < 'a , T , const CAP : usize , B : Behavior > Iterator for Drain < 'a , T , CAP , B > { type Item = T ; # [inline] fn next (& mut self) -> Option < T > { self . iter . next () . map (| elt | unsafe { ptr :: read (elt) }) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
