// Generated macro for impl_130 (impl)
macro_rules! Depcrate_filesimpl_130 {
() => {
// Module: crate::files
// Provides: {"impl_130"}
// Dependencies: {}
impl ErasedAstId { pub fn to_range (& self , db : & dyn ExpandDatabase) -> TextRange { self . to_ptr (db) . text_range () } pub fn to_ptr (& self , db : & dyn ExpandDatabase) -> SyntaxNodePtr { db . ast_id_map (self . file_id) . get_erased (self . value) } }
};
}
