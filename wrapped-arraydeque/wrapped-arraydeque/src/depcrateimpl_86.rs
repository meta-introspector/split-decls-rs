// Generated macro for impl_86 (impl)
macro_rules! Depcrateimpl_86 {
() => {
// Module: crate
// Provides: {"impl_86"}
// Dependencies: {}
impl < 'a , T , const CAP : usize , B : Behavior > DoubleEndedIterator for Drain < 'a , T , CAP , B > { # [inline] fn next_back (& mut self) -> Option < T > { self . iter . next_back () . map (| elt | unsafe { ptr :: read (elt) }) } }
};
}
