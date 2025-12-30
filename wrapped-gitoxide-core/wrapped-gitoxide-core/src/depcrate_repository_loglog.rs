// Generated macro for log (function)
macro_rules! Depcrate_repository_loglog {
() => {
// Module: crate::repository::log
// Provides: {"log"}
// Dependencies: {}
pub fn log (mut repo : gix :: Repository , out : & mut dyn std :: io :: Write , path : Option < BString >) -> anyhow :: Result < () > { repo . object_cache_size_if_unset (repo . compute_object_cache_size_for_tree_diffs (& * * repo . index_or_empty () ?)) ; if let Some (path) = path { log_file (repo , out , path) } else { log_all (repo , out) } }
};
}
