// Generated macro for AccessRelatedness (enum)
macro_rules! Depcrate_borrow_tracker_tree_borrows_treeAccessRelatedness {
() => {
// Module: crate::borrow_tracker::tree_borrows::tree
// Provides: {"AccessRelatedness"}
// Dependencies: {}
# [doc = " Relative position of the access"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum AccessRelatedness { # [doc = " The accessed pointer is the current one"] This , # [doc = " The accessed pointer is a (transitive) child of the current one."] StrictChildAccess , # [doc = " The accessed pointer is a (transitive) parent of the current one."] AncestorAccess , # [doc = " The accessed pointer is neither of the above."] CousinAccess , }
};
}
