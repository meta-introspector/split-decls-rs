// Generated macro for impl_24 (impl)
macro_rules! Depcrate_char16trie_trieimpl_24 {
() => {
// Module: crate::char16trie::trie
// Provides: {"impl_24"}
// Dependencies: {}
impl < 'data > Char16Trie < 'data > { # [doc = " Returns a new [`Char16Trie`] with ownership of the provided data."] # [inline] pub fn new (data : ZeroVec < 'data , u16 >) -> Self { Self { data } } # [doc = " Returns a new [`Char16TrieIterator`] backed by borrowed data from the `trie` data"] # [inline] pub fn iter (& self) -> Char16TrieIterator < '_ > { Char16TrieIterator :: new (& self . data) } }
};
}
