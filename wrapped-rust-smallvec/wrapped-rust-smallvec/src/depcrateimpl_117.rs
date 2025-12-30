// Generated macro for impl_117 (impl)
macro_rules! Depcrateimpl_117 {
() => {
// Module: crate
// Provides: {"impl_117"}
// Dependencies: {}
impl < 'a , T : 'a , const N : usize > Iterator for Drain < 'a , T , N > { type Item = T ; # [inline] fn next (& mut self) -> Option < T > { self . iter . next () . map (| reference | unsafe { core :: ptr :: read (reference) }) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
