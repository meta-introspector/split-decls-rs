// Generated macro for can_recurse (function)
macro_rules! Depcrate_walk_functioncan_recurse {
() => {
// Module: crate::walk::function
// Provides: {"can_recurse"}
// Dependencies: {}
pub (super) fn can_recurse (rela_path : & BStr , info : classify :: Outcome , for_deletion : Option < ForDeletionMode > , worktree_root_is_repository : bool , delegate : & mut dyn Delegate ,) -> bool { let is_dir = info . disk_kind . is_some_and (| k | k . is_dir ()) ; if ! is_dir { return false ; } delegate . can_recurse (EntryRef :: from_outcome (Cow :: Borrowed (rela_path) , info) , for_deletion , worktree_root_is_repository ,) }
};
}
