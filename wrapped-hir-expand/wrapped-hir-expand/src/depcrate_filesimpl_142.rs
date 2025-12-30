// Generated macro for impl_142 (impl)
macro_rules! Depcrate_filesimpl_142 {
() => {
// Module: crate::files
// Provides: {"impl_142"}
// Dependencies: {}
# [allow (private_bounds)] impl < FileId : FileIdToSyntax , T > InFileWrapper < FileId , T > { pub fn file_syntax (& self , db : & dyn db :: ExpandDatabase) -> SyntaxNode { FileIdToSyntax :: file_syntax (self . file_id , db) } }
};
}
