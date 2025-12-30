// Generated macro for impl_668 (impl)
macro_rules! Depcrate_nfa_thompson_range_trieimpl_668 {
() => {
// Module: crate::nfa::thompson::range_trie
// Provides: {"impl_668"}
// Dependencies: {}
impl fmt :: Debug for State { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let rs = self . transitions . iter () . map (| t | format ! ("{t:?}")) . collect :: < Vec < String > > () . join (", ") ; write ! (f , "{rs}") } }
};
}
