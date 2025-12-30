// Generated macro for impl_666 (impl)
macro_rules! Depcrate_nfa_thompson_range_trieimpl_666 {
() => {
// Module: crate::nfa::thompson::range_trie
// Provides: {"impl_666"}
// Dependencies: {}
impl fmt :: Debug for Transition { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . range . start == self . range . end { write ! (f , "{:02X} => {:02X}" , self . range . start , self . next_id . as_usize () ,) } else { write ! (f , "{:02X}-{:02X} => {:02X}" , self . range . start , self . range . end , self . next_id . as_usize () ,) } } }
};
}
