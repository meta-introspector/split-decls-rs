// Generated macro for edit_tree (module)
macro_rules! Depcrate_repositoryedit_tree {
() => {
// Module: crate::repository
// Provides: {"edit_tree"}
// Dependencies: {}
# [doc = ""] # [cfg (feature = "tree-editor")] pub mod edit_tree { # [doc = " The error returned by [Repository::edit_tree()](crate::Repository::edit_tree)."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] FindTree (# [from] crate :: object :: find :: existing :: with_conversion :: Error) , # [error (transparent)] InitEditor (# [from] crate :: object :: tree :: editor :: init :: Error) , } }
};
}
