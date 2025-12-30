// Generated macro for FileReference (struct)
macro_rules! Depcrate_searchFileReference {
() => {
// Module: crate::search
// Provides: {"FileReference"}
// Dependencies: {}
# [derive (Debug , Clone)] pub struct FileReference { # [doc = " The range of the reference in the original file"] pub range : TextRange , # [doc = " The node of the reference in the (macro-)file"] pub name : FileReferenceNode , pub category : ReferenceCategory , }
};
}
