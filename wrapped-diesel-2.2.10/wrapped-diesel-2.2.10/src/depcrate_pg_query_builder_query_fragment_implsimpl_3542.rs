// Generated macro for impl_3542 (impl)
macro_rules! Depcrate_pg_query_builder_query_fragment_implsimpl_3542 {
() => {
// Module: crate::pg::query_builder::query_fragment_impls
// Provides: {"impl_3542"}
// Dependencies: {}
impl < T , U > QueryFragment < Pg , crate :: pg :: backend :: PgOnConflictClause > for DecoratedConflictTarget < T , U > where T : QueryFragment < Pg > , U : QueryFragment < Pg > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Pg >) -> QueryResult < () > { self . target . walk_ast (out . reborrow ()) ? ; self . where_clause . walk_ast (out . reborrow ()) ? ; Ok (()) } }
};
}
