// Generated macro for impl_3504 (impl)
macro_rules! Depcrate_pg_query_builder_on_constraintimpl_3504 {
() => {
// Module: crate::pg::query_builder::on_constraint
// Provides: {"impl_3504"}
// Dependencies: {}
impl QueryFragment < Pg , crate :: pg :: backend :: PgOnConflictClause > for ConflictTarget < OnConstraint < '_ > > { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Pg >) -> QueryResult < () > { out . unsafe_to_cache_prepared () ; out . push_sql (" ON CONSTRAINT ") ; out . push_identifier (self . 0 . constraint_name) ? ; Ok (()) } }
};
}
