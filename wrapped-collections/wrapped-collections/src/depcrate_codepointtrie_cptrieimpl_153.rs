// Generated macro for impl_153 (impl)
macro_rules! Depcrate_codepointtrie_cptrieimpl_153 {
() => {
// Module: crate::codepointtrie::cptrie
// Provides: {"impl_153"}
// Dependencies: {}
impl < T : TrieValue + Into < u32 > > CodePointTrie < '_ , T > { # [doc = " Returns the value that is associated with `code_point` for this [`CodePointTrie`]"] # [doc = " as a `u32`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::collections::codepointtrie::planes;"] # [doc = " let trie = planes::get_planes_trie();"] # [doc = ""] # [doc = " let cp = '𑖎' as u32;"] # [doc = " assert_eq!(cp, 0x1158E);"] # [doc = ""] # [doc = " let plane_num: u8 = trie.get32(cp);"] # [doc = " assert_eq!(trie.get32_u32(cp), plane_num as u32);"] # [doc = " ```"] pub fn get32_u32 (& self , code_point : u32) -> u32 { self . get32 (code_point) . into () } }
};
}
