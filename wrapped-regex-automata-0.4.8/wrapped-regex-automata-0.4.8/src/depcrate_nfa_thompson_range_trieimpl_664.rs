// Generated macro for impl_664 (impl)
macro_rules! Depcrate_nfa_thompson_range_trieimpl_664 {
() => {
// Module: crate::nfa::thompson::range_trie
// Provides: {"impl_664"}
// Dependencies: {}
impl fmt :: Debug for RangeTrie { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { writeln ! (f , "") ? ; for (i , state) in self . states . iter () . enumerate () { let status = if i == FINAL . as_usize () { '*' } else { ' ' } ; writeln ! (f , "{}{:06}: {:?}" , status , i , state) ? ; } Ok (()) } }
};
}
