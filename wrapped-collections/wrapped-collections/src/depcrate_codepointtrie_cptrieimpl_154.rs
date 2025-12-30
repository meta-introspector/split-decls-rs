// Generated macro for impl_154 (impl)
macro_rules! Depcrate_codepointtrie_cptrieimpl_154 {
() => {
// Module: crate::codepointtrie::cptrie
// Provides: {"impl_154"}
// Dependencies: {}
impl < T : TrieValue > Clone for CodePointTrie < '_ , T > where < T as zerovec :: ule :: AsULE > :: ULE : Clone , { fn clone (& self) -> Self { CodePointTrie { header : self . header , index : self . index . clone () , data : self . data . clone () , error_value : self . error_value , } } }
};
}
