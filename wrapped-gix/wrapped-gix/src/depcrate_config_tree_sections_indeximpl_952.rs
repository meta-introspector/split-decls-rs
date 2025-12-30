// Generated macro for impl_952 (impl)
macro_rules! Depcrate_config_tree_sections_indeximpl_952 {
() => {
// Module: crate::config::tree::sections::index
// Provides: {"impl_952"}
// Dependencies: {}
impl Index { # [doc = " The `index.threads` key."] pub const THREADS : IndexThreads = IndexThreads :: new_with_validate ("threads" , & config :: Tree :: INDEX , validate :: IndexThreads) ; # [doc = " The `index.skipHash` key."] pub const SKIP_HASH : keys :: Boolean = keys :: Boolean :: new_boolean ("skipHash" , & config :: Tree :: INDEX) . with_deviation ("also used to skip the hash when reading, even if a hash exists in the index file") ; }
};
}
