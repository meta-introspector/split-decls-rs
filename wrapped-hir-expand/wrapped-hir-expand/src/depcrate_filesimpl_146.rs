// Generated macro for impl_146 (impl)
macro_rules! Depcrate_filesimpl_146 {
() => {
// Module: crate::files
// Provides: {"impl_146"}
// Dependencies: {}
impl < FileId : Copy , SN : Borrow < SyntaxNode > > InFileWrapper < FileId , SN > { pub fn file_range (& self) -> FileRangeWrapper < FileId > { FileRangeWrapper { file_id : self . file_id , range : self . value . borrow () . text_range () } } }
};
}
