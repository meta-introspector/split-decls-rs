// Generated macro for Error (enum)
macro_rules! Depcrate_tree_editorError {
() => {
// Module: crate::tree::editor
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [Editor] or [Cursor] edit operation."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Empty path components are not allowed")] EmptyPathComponent , # [error (transparent)] FindExistingObject (# [from] crate :: find :: existing_object :: Error) , }
};
}
