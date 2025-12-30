// Generated macro for impl_667 (impl)
macro_rules! Depcrate_nfa_thompson_range_trieimpl_667 {
() => {
// Module: crate::nfa::thompson::range_trie
// Provides: {"impl_667"}
// Dependencies: {}
impl fmt :: Debug for RangeTrie { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { writeln ! (f) ? ; for (i , state) in self . states . iter () . enumerate () { let status = if i == FINAL . as_usize () { '*' } else { ' ' } ; writeln ! (f , "{status}{i:06}: {state:?}") ? ; } Ok (()) } }
};
}
