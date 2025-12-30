// Generated macro for Error (enum)
macro_rules! Depcrate_tree_writeError {
() => {
// Module: crate::tree::write
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The Error used in [`Tree::write_to()`][crate::WriteTo::write_to()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Nullbytes are invalid in file paths as they are separators: {name:?}")] NullbyteInFilename { name : BString } , }
};
}
