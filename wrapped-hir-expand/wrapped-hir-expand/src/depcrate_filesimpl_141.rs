// Generated macro for impl_141 (impl)
macro_rules! Depcrate_filesimpl_141 {
() => {
// Module: crate::files
// Provides: {"impl_141"}
// Dependencies: {}
impl FileIdToSyntax for HirFileId { fn file_syntax (self , db : & dyn db :: ExpandDatabase) -> SyntaxNode { db . parse_or_expand (self) } }
};
}
