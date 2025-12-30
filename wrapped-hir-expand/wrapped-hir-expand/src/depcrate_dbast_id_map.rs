// Generated macro for ast_id_map (function)
macro_rules! Depcrate_dbast_id_map {
() => {
// Module: crate::db
// Provides: {"ast_id_map"}
// Dependencies: {}
fn ast_id_map (db : & dyn ExpandDatabase , file_id : HirFileId) -> triomphe :: Arc < AstIdMap > { triomphe :: Arc :: new (AstIdMap :: from_source (& db . parse_or_expand (file_id))) }
};
}
