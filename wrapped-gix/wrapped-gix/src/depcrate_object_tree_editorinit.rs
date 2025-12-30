// Generated macro for init (module)
macro_rules! Depcrate_object_tree_editorinit {
() => {
// Module: crate::object::tree::editor
// Provides: {"init"}
// Dependencies: {}
# [doc = ""] pub mod init { # [doc = " The error returned by [`Editor::new()](crate::object::tree::Editor::new())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] DecodeTree (# [from] gix_object :: decode :: Error) , # [error (transparent)] ValidationOptions (# [from] crate :: config :: boolean :: Error) , } }
};
}
