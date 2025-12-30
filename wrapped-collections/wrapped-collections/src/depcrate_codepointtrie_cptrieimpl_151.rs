// Generated macro for impl_151 (impl)
macro_rules! Depcrate_codepointtrie_cptrieimpl_151 {
() => {
// Module: crate::codepointtrie::cptrie
// Provides: {"impl_151"}
// Dependencies: {}
# [cfg (feature = "databake")] impl < T : TrieValue + databake :: Bake > databake :: Bake for CodePointTrie < '_ , T > { fn bake (& self , env : & databake :: CrateEnv) -> databake :: TokenStream { let header = self . header . bake (env) ; let index = self . index . bake (env) ; let data = self . data . bake (env) ; let error_value = self . error_value . bake (env) ; databake :: quote ! { unsafe { icu_collections :: codepointtrie :: CodePointTrie :: from_parts_unstable_unchecked_v1 (# header , # index , # data , # error_value) } } } }
};
}
