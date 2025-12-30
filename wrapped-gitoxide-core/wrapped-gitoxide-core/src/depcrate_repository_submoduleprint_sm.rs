// Generated macro for print_sm (function)
macro_rules! Depcrate_repository_submoduleprint_sm {
() => {
// Module: crate::repository::submodule
// Provides: {"print_sm"}
// Dependencies: {}
fn print_sm (sm : Submodule < '_ > , dirty_suffix : Option < & str > , out : & mut impl std :: io :: Write) -> anyhow :: Result < () > { let _span = gix :: trace :: coarse ! ("print_sm" , path = ? sm . path ()) ; let state = sm . state () ? ; let mut sm_repo = sm . open () ? ; if let Some (repo) = sm_repo . as_mut () { repo . object_cache_size_if_unset (4 * 1024 * 1024) ; } writeln ! (out , " {is_active} {path:?} {config} head:{head_id} index:{index_id} ({worktree}) [{url}]" , is_active = if ! sm . is_active () ? || ! state . repository_exists { "ⅹ" } else { "✓" } , path = sm . path () ?, config = if state . superproject_configuration { "config:yes" } else { "config:no" } , head_id = submodule_short_hash (sm . head_id () ?, sm_repo . as_ref ()) , index_id = submodule_short_hash (sm . index_id () ?, sm_repo . as_ref ()) , worktree = match sm_repo { Some (repo) => { repo . head_commit () ? . describe () . names (SelectRef :: AllRefs) . id_as_fallback (true) . try_resolve () ? . expect ("resolution present if ID can be used as fallback") . format_with_dirty_suffix (dirty_suffix . map (ToOwned :: to_owned)) ? . to_string () } None => { "no worktree" . into () } } , url = sm . url () ?. to_bstring ()) ? ; Ok (()) }
};
}
