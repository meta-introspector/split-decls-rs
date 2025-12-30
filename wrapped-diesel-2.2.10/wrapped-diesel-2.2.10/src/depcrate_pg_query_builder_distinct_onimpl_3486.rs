// Generated macro for impl_3486 (impl)
macro_rules! Depcrate_pg_query_builder_distinct_onimpl_3486 {
() => {
// Module: crate::pg::query_builder::distinct_on
// Provides: {"impl_3486"}
// Dependencies: {}
impl < T > QueryFragment < Pg > for DistinctOnClause < T > where T : QueryFragment < Pg > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Pg >) -> QueryResult < () > { out . push_sql ("DISTINCT ON (") ; self . 0 . walk_ast (out . reborrow ()) ? ; out . push_sql (") ") ; Ok (()) } }
};
}
