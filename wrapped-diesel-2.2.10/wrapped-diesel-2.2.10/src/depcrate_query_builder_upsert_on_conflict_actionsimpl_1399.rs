// Generated macro for impl_1399 (impl)
macro_rules! Depcrate_query_builder_upsert_on_conflict_actionsimpl_1399 {
() => {
// Module: crate::query_builder::upsert::on_conflict_actions
// Provides: {"impl_1399"}
// Dependencies: {}
impl < T , DB > QueryFragment < DB > for DoNothing < T > where DB : Backend , Self : QueryFragment < DB , DB :: OnConflictClause > , { fn walk_ast < 'b > (& 'b self , pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { < Self as QueryFragment < DB , DB :: OnConflictClause > > :: walk_ast (self , pass) } }
};
}
