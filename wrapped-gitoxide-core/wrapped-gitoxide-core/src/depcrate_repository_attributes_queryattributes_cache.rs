// Generated macro for attributes_cache (function)
macro_rules! Depcrate_repository_attributes_queryattributes_cache {
() => {
// Module: crate::repository::attributes::query
// Provides: {"attributes_cache"}
// Dependencies: {}
pub (crate) fn attributes_cache (repo : & gix :: Repository ,) -> anyhow :: Result < (gix :: AttributeStack < '_ > , IndexPersistedOrInMemory) > { let index = repo . index_or_load_from_head () ? ; let cache = repo . attributes (& index , if repo . is_bare () { gix :: worktree :: stack :: state :: attributes :: Source :: IdMapping } else { gix :: worktree :: stack :: state :: attributes :: Source :: WorktreeThenIdMapping } , gix :: worktree :: stack :: state :: ignore :: Source :: IdMapping , None ,) ? ; Ok ((cache , index)) }
};
}
