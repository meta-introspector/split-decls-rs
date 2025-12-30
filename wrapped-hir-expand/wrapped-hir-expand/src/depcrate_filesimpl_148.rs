// Generated macro for impl_148 (impl)
macro_rules! Depcrate_filesimpl_148 {
() => {
// Module: crate::files
// Provides: {"impl_148"}
// Dependencies: {}
impl InFile < & SyntaxNode > { # [doc = " Attempts to map the syntax node back up its macro calls."] pub fn original_file_range_opt (self , db : & dyn db :: ExpandDatabase ,) -> Option < (FileRange , SyntaxContext) > { self . borrow () . map (SyntaxNode :: text_range) . original_node_file_range_opt (db) } }
};
}
