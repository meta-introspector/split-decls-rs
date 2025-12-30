// Generated macro for impl_143 (impl)
macro_rules! Depcrate_filesimpl_143 {
() => {
// Module: crate::files
// Provides: {"impl_143"}
// Dependencies: {}
# [allow (private_bounds)] impl < FileId : FileIdToSyntax , N : AstNode > InFileWrapper < FileId , AstPtr < N > > { pub fn to_node (& self , db : & dyn ExpandDatabase) -> N { self . value . to_node (& self . file_syntax (db)) } }
};
}
