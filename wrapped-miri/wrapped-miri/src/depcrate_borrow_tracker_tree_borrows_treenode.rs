// Generated macro for Node (struct)
macro_rules! Depcrate_borrow_tracker_tree_borrows_treeNode {
() => {
// Module: crate::borrow_tracker::tree_borrows::tree
// Provides: {"Node"}
// Dependencies: {}
# [doc = " A node in the borrow tree. Each node is uniquely identified by a tag via"] # [doc = " the `nodes` map of `Tree`."] # [derive (Clone , Debug)] pub (super) struct Node { # [doc = " The tag of this node."] pub tag : BorTag , # [doc = " All tags except the root have a parent tag."] pub parent : Option < UniIndex > , # [doc = " If the pointer was reborrowed, it has children."] pub children : SmallVec < [UniIndex ; 4] > , # [doc = " Either `Reserved`,  `Frozen`, or `Disabled`, it is the permission this tag will"] # [doc = " lazily be initialized to on the first access."] # [doc = " It is only ever `Disabled` for a tree root, since the root is initialized to `Active` by"] # [doc = " its own separate mechanism."] default_initial_perm : Permission , # [doc = " The default initial (strongest) idempotent foreign access."] # [doc = " This participates in the invariant for `LocationState::idempotent_foreign_access`"] # [doc = " in cases where there is no location state yet. See `foreign_access_skipping.rs`,"] # [doc = " and `LocationState::idempotent_foreign_access` for more information"] default_initial_idempotent_foreign_access : IdempotentForeignAccess , # [doc = " Some extra information useful only for debugging purposes"] pub debug_info : NodeDebugInfo , }
};
}
