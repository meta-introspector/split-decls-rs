// Generated macro for impl_579 (impl)
macro_rules! Depcrate_nfa_thompson_literal_trieimpl_579 {
() => {
// Module: crate::nfa::thompson::literal_trie
// Provides: {"impl_579"}
// Dependencies: {}
impl core :: fmt :: Debug for Transition { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "{:?} => {}" , crate :: util :: escape :: DebugByte (self . byte) , self . next . as_usize ()) } }
};
}
