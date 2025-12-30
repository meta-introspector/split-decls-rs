// Generated macro for impl_3537 (impl)
macro_rules! Depcrate_pg_query_builder_query_fragment_implsimpl_3537 {
() => {
// Module: crate::pg::query_builder::query_fragment_impls
// Provides: {"impl_3537"}
// Dependencies: {}
impl QueryFragment < Pg > for SkipLocked { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Pg >) -> QueryResult < () > { out . push_sql (" SKIP LOCKED") ; Ok (()) } }
};
}
