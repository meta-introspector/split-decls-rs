// Generated macro for impl_1436 (impl)
macro_rules! Depcrate_query_builder_upsert_on_conflict_targetimpl_1436 {
() => {
// Module: crate::query_builder::upsert::on_conflict_target
// Provides: {"impl_1436"}
// Dependencies: {}
impl < DB , T > QueryFragment < DB > for ConflictTarget < T > where DB : Backend , Self : QueryFragment < DB , DB :: OnConflictClause > , { fn walk_ast < 'b > (& 'b self , pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { < Self as QueryFragment < DB , DB :: OnConflictClause > > :: walk_ast (self , pass) } }
};
}
