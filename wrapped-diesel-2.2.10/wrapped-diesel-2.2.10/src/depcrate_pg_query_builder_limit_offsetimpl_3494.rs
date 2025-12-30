// Generated macro for impl_3494 (impl)
macro_rules! Depcrate_pg_query_builder_limit_offsetimpl_3494 {
() => {
// Module: crate::pg::query_builder::limit_offset
// Provides: {"impl_3494"}
// Dependencies: {}
impl QueryFragment < Pg > for BoxedLimitOffsetClause < '_ , Pg > { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Pg >) -> QueryResult < () > { if let Some (ref limit) = self . limit { limit . walk_ast (out . reborrow ()) ? ; } if let Some (ref offset) = self . offset { offset . walk_ast (out . reborrow ()) ? ; } Ok (()) } }
};
}
