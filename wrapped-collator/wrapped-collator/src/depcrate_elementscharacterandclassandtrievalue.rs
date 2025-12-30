// Generated macro for CharacterAndClassAndTrieValue (struct)
macro_rules! Depcrate_elementsCharacterAndClassAndTrieValue {
() => {
// Module: crate::elements
// Provides: {"CharacterAndClassAndTrieValue"}
// Dependencies: {}
# [doc = " This struct makes the handling of the `upcoming` buffer"] # [doc = " easily so that trie lookups are done at most once. However,"] # [doc = " when `upcoming[0]` is an undecomposed starter, we don't"] # [doc = " need the ccc yet, and when lookahead has already done the"] # [doc = " trie lookups, we don't need `trie_value`, as it is implied"] # [doc = " by ccc."] # [derive (Debug , Clone)] pub (crate) struct CharacterAndClassAndTrieValue { c_and_c : CharacterAndClass , pub trie_val : u32 , }
};
}
