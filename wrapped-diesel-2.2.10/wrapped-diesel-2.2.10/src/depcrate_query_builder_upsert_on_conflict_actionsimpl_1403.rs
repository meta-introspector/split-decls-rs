// Generated macro for impl_1403 (impl)
macro_rules! Depcrate_query_builder_upsert_on_conflict_actionsimpl_1403 {
() => {
// Module: crate::query_builder::upsert::on_conflict_actions
// Provides: {"impl_1403"}
// Dependencies: {}
impl < DB , T , Tab > QueryFragment < DB > for DoUpdate < T , Tab > where DB : Backend , Self : QueryFragment < DB , DB :: OnConflictClause > , { fn walk_ast < 'b > (& 'b self , pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { < Self as QueryFragment < DB , DB :: OnConflictClause > > :: walk_ast (self , pass) } }
};
}
