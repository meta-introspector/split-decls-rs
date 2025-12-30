// Generated macro for impl_169 (impl)
macro_rules! Depcrate_access_sparseimpl_169 {
() => {
// Module: crate::access::sparse
// Provides: {"impl_169"}
// Dependencies: {}
impl Options { # [doc = " Derive a valid mode from all parameters that affect the 'sparseness' of the index."] # [doc = ""] # [doc = " Some combinations of them degenerate to one particular mode."] pub fn sparse_mode (& self) -> Mode { match (self . sparse_checkout , self . directory_patterns_only , self . write_sparse_index ,) { (true , true , true) => Mode :: IncludeDirectoriesStoreIncludedEntriesAndExcludedDirs , (true , true , false) => Mode :: IncludeDirectoriesStoreAllEntriesSkipUnmatched , (true , false , _) => Mode :: IncludeByIgnorePatternStoreAllEntriesSkipUnmatched , (false , _ , _) => Mode :: Disabled , } } }
};
}
