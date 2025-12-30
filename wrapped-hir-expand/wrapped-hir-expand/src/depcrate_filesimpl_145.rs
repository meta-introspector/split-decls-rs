// Generated macro for impl_145 (impl)
macro_rules! Depcrate_filesimpl_145 {
() => {
// Module: crate::files
// Provides: {"impl_145"}
// Dependencies: {}
impl < FileId : Copy , N : AstNode > InFileWrapper < FileId , & N > { pub fn syntax_ref (& self) -> InFileWrapper < FileId , & SyntaxNode > { self . with_value (self . value . syntax ()) } }
};
}
