// Generated macro for impl_139 (impl)
macro_rules! Depcrate_filesimpl_139 {
() => {
// Module: crate::files
// Provides: {"impl_139"}
// Dependencies: {}
impl FileIdToSyntax for EditionedFileId { fn file_syntax (self , db : & dyn db :: ExpandDatabase) -> SyntaxNode { db . parse (self) . syntax_node () } }
};
}
