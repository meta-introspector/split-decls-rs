// Generated macro for u16_tail (function)
macro_rules! Depcrate_char16trie_trieu16_tail {
() => {
// Module: crate::char16trie::trie
// Provides: {"u16_tail"}
// Dependencies: {}
fn u16_tail (supplementary : i32) -> u16 { (((supplementary) & 0x3ff) | 0xdc00) as u16 }
};
}
