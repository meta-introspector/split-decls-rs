// Generated macro for Char16TrieIterator (struct)
macro_rules! Depcrate_char16trie_trieChar16TrieIterator {
() => {
// Module: crate::char16trie::trie
// Provides: {"Char16TrieIterator"}
// Dependencies: {}
# [doc = " This struct represents an iterator over a [`Char16Trie`]."] # [derive (Clone)] pub struct Char16TrieIterator < 'a > { # [doc = " A reference to the Char16Trie data to iterate over."] trie : & 'a ZeroSlice < u16 > , # [doc = " Index of next trie unit to read, or `None` if there are no more matches."] pos : Option < usize > , # [doc = " Remaining length of a linear-match node, minus 1, or `None` if not in"] # [doc = " such a node."] remaining_match_length : Option < usize > , }
};
}
