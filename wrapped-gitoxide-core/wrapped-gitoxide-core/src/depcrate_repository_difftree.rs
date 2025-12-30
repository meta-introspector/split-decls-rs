// Generated macro for tree (function)
macro_rules! Depcrate_repository_difftree {
() => {
// Module: crate::repository::diff
// Provides: {"tree"}
// Dependencies: {}
pub fn tree (mut repo : gix :: Repository , out : & mut dyn std :: io :: Write , old_treeish : BString , new_treeish : BString ,) -> anyhow :: Result < () > { repo . object_cache_size_if_unset (repo . compute_object_cache_size_for_tree_diffs (& * * repo . index_or_empty () ?)) ; repo . objects . refresh = RefreshMode :: Never ; let old_tree_id = repo . rev_parse_single (old_treeish . as_bstr ()) ? ; let new_tree_id = repo . rev_parse_single (new_treeish . as_bstr ()) ? ; let old_tree = old_tree_id . object () ? . peel_to_tree () ? ; let new_tree = new_tree_id . object () ? . peel_to_tree () ? ; let changes = repo . diff_tree_to_tree (& old_tree , & new_tree , None) ? ; writeln ! (out , "Diffing trees `{old_treeish}` ({old_tree_id}) -> `{new_treeish}` ({new_tree_id})\n") ? ; write_changes (& repo , out , changes) ? ; Ok (()) }
};
}
