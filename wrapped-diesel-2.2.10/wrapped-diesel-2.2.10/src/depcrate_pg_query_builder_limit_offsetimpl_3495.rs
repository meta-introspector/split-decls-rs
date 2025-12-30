// Generated macro for impl_3495 (impl)
macro_rules! Depcrate_pg_query_builder_limit_offsetimpl_3495 {
() => {
// Module: crate::pg::query_builder::limit_offset
// Provides: {"impl_3495"}
// Dependencies: {}
impl < L , O > QueryFragment < Pg > for LimitOffsetClause < L , O > where L : QueryFragment < Pg > , O : QueryFragment < Pg > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Pg >) -> QueryResult < () > { self . limit_clause . walk_ast (out . reborrow ()) ? ; self . offset_clause . walk_ast (out . reborrow ()) ? ; Ok (()) } }
};
}
