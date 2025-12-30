// Generated macro for blame_file (function)
macro_rules! Depcrate_repository_blameblame_file {
() => {
// Module: crate::repository::blame
// Provides: {"blame_file"}
// Dependencies: {}
pub fn blame_file (mut repo : gix :: Repository , file : & OsStr , options : gix :: blame :: Options , out : impl std :: io :: Write , err : Option < & mut dyn std :: io :: Write > ,) -> anyhow :: Result < () > { { let mut config = repo . config_snapshot_mut () ; if config . string (& tree :: Core :: DELTA_BASE_CACHE_LIMIT) . is_none () { config . set_value (& tree :: Core :: DELTA_BASE_CACHE_LIMIT , "100m") ? ; } } let index = repo . index_or_empty () ? ; repo . object_cache_size_if_unset (repo . compute_object_cache_size_for_tree_diffs (& index)) ; let file = gix :: path :: os_str_into_bstr (file) ? ; let specs = repo . pathspec (false , [file] , true , & index , gix :: worktree :: stack :: state :: attributes :: Source :: WorktreeThenIdMapping . adjust_for_bare (repo . is_bare ()) ,) ? ; let file = specs . search () . patterns () . map (| p | p . path () . to_owned ()) . next () . expect ("exactly one pattern") ; let suspect : gix :: ObjectId = repo . head () ? . into_peeled_id () ? . into () ; let cache : Option < gix :: commitgraph :: Graph > = repo . commit_graph_if_enabled () ? ; let mut resource_cache = repo . diff_resource_cache_for_tree_diff () ? ; let outcome = gix :: blame :: file (& repo . objects , suspect , cache , & mut resource_cache , file . as_bstr () , options ,) ? ; let statistics = outcome . statistics ; show_blame_entries (out , outcome , file) ? ; if let Some (err) = err { writeln ! (err , "{statistics:#?}") ? ; } Ok (()) }
};
}
