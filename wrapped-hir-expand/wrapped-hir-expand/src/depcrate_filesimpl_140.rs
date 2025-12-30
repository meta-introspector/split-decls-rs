// Generated macro for impl_140 (impl)
macro_rules! Depcrate_filesimpl_140 {
() => {
// Module: crate::files
// Provides: {"impl_140"}
// Dependencies: {}
impl FileIdToSyntax for MacroCallId { fn file_syntax (self , db : & dyn db :: ExpandDatabase) -> SyntaxNode { db . parse_macro_expansion (self) . value . 0 . syntax_node () } }
};
}
