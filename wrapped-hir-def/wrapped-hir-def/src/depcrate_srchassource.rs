// Generated macro for HasSource (trait)
macro_rules! Depcrate_srcHasSource {
() => {
// Module: crate::src
// Provides: {"HasSource"}
// Dependencies: {}
pub trait HasSource { type Value : AstNode ; fn source (& self , db : & dyn DefDatabase) -> InFile < Self :: Value > { let InFile { file_id , value } = self . ast_ptr (db) ; InFile :: new (file_id , value . to_node (& db . parse_or_expand (file_id))) } fn ast_ptr (& self , db : & dyn DefDatabase) -> InFile < AstPtr < Self :: Value > > ; }
};
}
