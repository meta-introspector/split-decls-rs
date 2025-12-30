// Generated macro for impl_118 (impl)
macro_rules! Depcrateimpl_118 {
() => {
// Module: crate
// Provides: {"impl_118"}
// Dependencies: {}
impl < 'a , T : 'a , const N : usize > DoubleEndedIterator for Drain < 'a , T , N > { # [inline] fn next_back (& mut self) -> Option < T > { self . iter . next_back () . map (| reference | unsafe { core :: ptr :: read (reference) }) } }
};
}
