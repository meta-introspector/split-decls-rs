// Generated macro for impl_1005 (impl)
macro_rules! Depcrate_query_builder_insert_statement_insert_with_default_for_sqliteimpl_1005 {
() => {
// Module: crate::query_builder::insert_statement::insert_with_default_for_sqlite
// Provides: {"impl_1005"}
// Dependencies: {}
impl < V , Tab , QId , const STATIC_QUERY_ID : bool > QueryFragment < Sqlite > for SqliteBatchInsertWrapper < Vec < ValuesClause < V , Tab > > , Tab , QId , STATIC_QUERY_ID > where ValuesClause < V , Tab > : QueryFragment < Sqlite > , V : QueryFragment < Sqlite > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Sqlite >) -> QueryResult < () > { if ! STATIC_QUERY_ID { out . unsafe_to_cache_prepared () ; } let mut values = self . 0 . values . iter () ; if let Some (value) = values . next () { value . walk_ast (out . reborrow ()) ? ; } for value in values { out . push_sql (", (") ; value . values . walk_ast (out . reborrow ()) ? ; out . push_sql (")") ; } Ok (()) } }
};
}
