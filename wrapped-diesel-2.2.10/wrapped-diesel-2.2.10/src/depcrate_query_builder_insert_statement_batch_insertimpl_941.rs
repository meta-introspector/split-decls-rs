// Generated macro for impl_941 (impl)
macro_rules! Depcrate_query_builder_insert_statement_batch_insertimpl_941 {
() => {
// Module: crate::query_builder::insert_statement::batch_insert
// Provides: {"impl_941"}
// Dependencies: {}
impl < Tab , DB , V , QId , const HAS_STATIC_QUERY_ID : bool > QueryFragment < DB , sql_dialect :: batch_insert_support :: PostgresLikeBatchInsertSupport > for BatchInsert < Vec < ValuesClause < V , Tab > > , Tab , QId , HAS_STATIC_QUERY_ID > where DB : Backend + SqlDialect < BatchInsertSupport = sql_dialect :: batch_insert_support :: PostgresLikeBatchInsertSupport , > , DB :: InsertWithDefaultKeyword : sql_dialect :: default_keyword_for_insert :: SupportsDefaultKeyword , ValuesClause < V , Tab > : QueryFragment < DB > , V : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { if ! HAS_STATIC_QUERY_ID { out . unsafe_to_cache_prepared () ; } let mut values = self . values . iter () ; if let Some (value) = values . next () { value . walk_ast (out . reborrow ()) ? ; } for value in values { out . push_sql (", (") ; value . values . walk_ast (out . reborrow ()) ? ; out . push_sql (")") ; } Ok (()) } }
};
}
