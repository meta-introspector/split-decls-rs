// Generated macro for impl_15 (impl)
macro_rules! Depcrate_index_as_worktree_typesimpl_15 {
() => {
// Module: crate::index_as_worktree::types
// Provides: {"impl_15"}
// Dependencies: {}
impl From < & gix_index :: Entry > for ConflictIndexEntry { fn from (gix_index :: Entry { stat : _ , id , flags , mode , .. } : & gix_index :: Entry ,) -> Self { ConflictIndexEntry { id : * id , flags : * flags , mode : * mode , } } }
};
}
