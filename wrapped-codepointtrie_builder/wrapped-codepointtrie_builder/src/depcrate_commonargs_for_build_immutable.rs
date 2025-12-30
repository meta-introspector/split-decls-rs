// Generated macro for args_for_build_immutable (function)
macro_rules! Depcrate_commonargs_for_build_immutable {
() => {
// Module: crate::common
// Provides: {"args_for_build_immutable"}
// Dependencies: {}
# [doc = " Returns the type and width arguments for `umutablecptrie_buildImmutable`"] pub (crate) fn args_for_build_immutable < U > (trie_type : TrieType) -> (u32 , u32) { let trie_type = match trie_type { TrieType :: Fast => 0 , TrieType :: Small => 1 , } ; let width = match core :: mem :: size_of :: < U > () { 1 => 2 , 2 => 0 , 3 | 4 => 1 , other => panic ! ("Don't know how to make trie with width {other}") , } ; (trie_type , width) }
};
}
