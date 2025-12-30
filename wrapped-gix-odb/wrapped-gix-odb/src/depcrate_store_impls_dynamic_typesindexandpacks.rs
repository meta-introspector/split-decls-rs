// Generated macro for IndexAndPacks (enum)
macro_rules! Depcrate_store_impls_dynamic_typesIndexAndPacks {
() => {
// Module: crate::store_impls::dynamic::types
// Provides: {"IndexAndPacks"}
// Dependencies: {}
# [derive (Clone)] pub (crate) enum IndexAndPacks { Index (IndexFileBundle) , # [doc = " Note that there can only be one multi-pack file per repository, but thanks to git alternates, there can be multiple overall."] MultiIndex (MultiIndexFileBundle) , }
};
}
