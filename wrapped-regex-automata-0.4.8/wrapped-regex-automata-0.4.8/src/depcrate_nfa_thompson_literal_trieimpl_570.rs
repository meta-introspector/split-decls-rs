// Generated macro for impl_570 (impl)
macro_rules! Depcrate_nfa_thompson_literal_trieimpl_570 {
() => {
// Module: crate::nfa::thompson::literal_trie
// Provides: {"impl_570"}
// Dependencies: {}
impl core :: fmt :: Debug for LiteralTrie { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { writeln ! (f , "LiteralTrie(") ? ; for (sid , state) in self . states . iter () . with_state_ids () { writeln ! (f , "{:06?}: {:?}" , sid . as_usize () , state) ? ; } writeln ! (f , ")") ? ; Ok (()) } }
};
}
