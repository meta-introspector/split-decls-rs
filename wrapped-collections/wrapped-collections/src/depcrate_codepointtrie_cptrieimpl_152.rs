// Generated macro for impl_152 (impl)
macro_rules! Depcrate_codepointtrie_cptrieimpl_152 {
() => {
// Module: crate::codepointtrie::cptrie
// Provides: {"impl_152"}
// Dependencies: {}
# [cfg (feature = "databake")] impl < T : TrieValue + databake :: Bake > databake :: BakeSize for CodePointTrie < '_ , T > { fn borrows_size (& self) -> usize { self . header . borrows_size () + self . index . borrows_size () + self . data . borrows_size () } }
};
}
