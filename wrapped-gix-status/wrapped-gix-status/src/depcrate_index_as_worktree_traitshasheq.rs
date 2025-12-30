// Generated macro for HashEq (struct)
macro_rules! Depcrate_index_as_worktree_traitsHashEq {
() => {
// Module: crate::index_as_worktree::traits
// Provides: {"HashEq"}
// Dependencies: {}
# [doc = " Compares files to blobs by *always* comparing their hashes."] # [doc = ""] # [doc = " Same as [`FastEq`] but does not contain a fast path for files with mismatched files and"] # [doc = " therefore always returns an OID that can be reused later."] # [derive (Clone)] pub struct HashEq ;
};
}
