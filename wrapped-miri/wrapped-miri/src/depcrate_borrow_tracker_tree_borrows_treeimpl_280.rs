// Generated macro for impl_280 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_treeimpl_280 {
() => {
// Module: crate::borrow_tracker::tree_borrows::tree
// Provides: {"impl_280"}
// Dependencies: {}
impl Tree { # [doc = " Create a new tree, with only a root pointer."] pub fn new (root_tag : BorTag , size : Size , span : Span) -> Self { let root_default_perm = Permission :: new_disabled () ; let mut tag_mapping = UniKeyMap :: default () ; let root_idx = tag_mapping . insert (root_tag) ; let nodes = { let mut nodes = UniValMap :: < Node > :: default () ; let mut debug_info = NodeDebugInfo :: new (root_tag , root_default_perm , span) ; debug_info . add_name ("root of the allocation") ; nodes . insert (root_idx , Node { tag : root_tag , parent : None , children : SmallVec :: default () , default_initial_perm : root_default_perm , default_initial_idempotent_foreign_access : IdempotentForeignAccess :: None , debug_info , } ,) ; nodes } ; let rperms = { let mut perms = UniValMap :: default () ; perms . insert (root_idx , LocationState :: new_accessed (Permission :: new_active () , IdempotentForeignAccess :: None ,) ,) ; DedupRangeMap :: new (size , perms) } ; Self { root : root_idx , nodes , rperms , tag_mapping } } }
};
}
