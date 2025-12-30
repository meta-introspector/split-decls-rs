// Generated macro for impl_157 (impl)
macro_rules! Depcrate_codepointtrie_cptrieimpl_157 {
() => {
// Module: crate::codepointtrie::cptrie
// Provides: {"impl_157"}
// Dependencies: {}
impl < T : TrieValue > Iterator for CodePointMapRangeIterator < '_ , T > { type Item = CodePointMapRange < T > ; fn next (& mut self) -> Option < Self :: Item > { self . cpm_range = match & self . cpm_range { Some (cpmr) => { if * cpmr . range . start () == u32 :: MAX { self . cpt . get_range (0) } else { self . cpt . get_range (cpmr . range . end () + 1) } } None => None , } ; self . cpm_range . clone () } }
};
}
