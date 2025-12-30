// Generated macro for impl_1423 (impl)
macro_rules! Depcrate_query_builder_upsert_on_conflict_clauseimpl_1423 {
() => {
// Module: crate::query_builder::upsert::on_conflict_clause
// Provides: {"impl_1423"}
// Dependencies: {}
impl < DB , Values , Target , Action > QueryFragment < DB > for OnConflictValues < Values , Target , Action , NoWhereClause > where DB : Backend , DB :: OnConflictClause : sql_dialect :: on_conflict_clause :: SupportsOnConflictClause , Self : QueryFragment < DB , DB :: OnConflictClause > , { fn walk_ast < 'b > (& 'b self , pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { < Self as QueryFragment < DB , DB :: OnConflictClause > > :: walk_ast (self , pass) } }
};
}
