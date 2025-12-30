// Generated macro for impl_3533 (impl)
macro_rules! Depcrate_pg_query_builder_query_fragment_implsimpl_3533 {
() => {
// Module: crate::pg::query_builder::query_fragment_impls
// Provides: {"impl_3533"}
// Dependencies: {}
impl QueryFragment < Pg > for ForNoKeyUpdate { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Pg >) -> QueryResult < () > { out . push_sql (" FOR NO KEY UPDATE") ; Ok (()) } }
};
}
