// Generated macro for impl_1372 (impl)
macro_rules! Depcrate_query_builder_update_statementimpl_1372 {
() => {
// Module: crate::query_builder::update_statement
// Provides: {"impl_1372"}
// Dependencies: {}
impl < T , U , V , Ret , DB > QueryFragment < DB > for UpdateStatement < T , U , V , Ret > where DB : Backend + DieselReserveSpecialization , T : Table , T :: FromClause : QueryFragment < DB > , U : QueryFragment < DB > , V : QueryFragment < DB > , Ret : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { if self . values . is_noop (out . backend ()) ? { return Err (QueryBuilderError (Box :: new (EmptyChangeset))) ; } out . unsafe_to_cache_prepared () ; out . push_sql ("UPDATE ") ; self . from_clause . walk_ast (out . reborrow ()) ? ; out . push_sql (" SET ") ; self . values . walk_ast (out . reborrow ()) ? ; self . where_clause . walk_ast (out . reborrow ()) ? ; self . returning . walk_ast (out . reborrow ()) ? ; Ok (()) } }
};
}
