// Generated macro for impl_1407 (impl)
macro_rules! Depcrate_query_builder_upsert_on_conflict_actionsimpl_1407 {
() => {
// Module: crate::query_builder::upsert::on_conflict_actions
// Provides: {"impl_1407"}
// Dependencies: {}
impl < DB , T > QueryFragment < DB > for Excluded < T > where DB : Backend , Self : QueryFragment < DB , DB :: OnConflictClause > , { fn walk_ast < 'b > (& 'b self , pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { < Self as QueryFragment < DB , DB :: OnConflictClause > > :: walk_ast (self , pass) } }
};
}
