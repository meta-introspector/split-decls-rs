// Generated macro for write (module)
macro_rules! Depcrate_object_tree_editorwrite {
() => {
// Module: crate::object::tree::editor
// Provides: {"write"}
// Dependencies: {}
# [doc = ""] pub mod write { use crate :: bstr :: BString ; # [doc = " The error returned by [`Editor::write()](crate::object::tree::Editor::write()) and [`Cursor::write()](super::Cursor::write)."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] WriteTree (# [from] crate :: object :: write :: Error) , # [error ("The object {} ({}) at '{}' could not be found" , id , kind . as_octal_str () , filename)] MissingObject { filename : BString , kind : gix_object :: tree :: EntryKind , id : gix_hash :: ObjectId , } , # [error ("The object {} ({}) has an invalid filename: '{}'" , id , kind . as_octal_str () , filename)] InvalidFilename { filename : BString , kind : gix_object :: tree :: EntryKind , id : gix_hash :: ObjectId , source : gix_validate :: path :: component :: Error , } , } }
};
}
