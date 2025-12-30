// Generated macro for impl_144 (impl)
macro_rules! Depcrate_filesimpl_144 {
() => {
// Module: crate::files
// Provides: {"impl_144"}
// Dependencies: {}
impl < FileId : Copy , N : AstNode > InFileWrapper < FileId , N > { pub fn syntax (& self) -> InFileWrapper < FileId , & SyntaxNode > { self . with_value (self . value . syntax ()) } pub fn node_file_range (& self) -> FileRangeWrapper < FileId > { FileRangeWrapper { file_id : self . file_id , range : self . value . syntax () . text_range () } } }
};
}
