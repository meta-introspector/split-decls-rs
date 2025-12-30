// Generated macro for impl_3895 (impl)
macro_rules! Depcrate_sqlite_query_builder_limit_offsetimpl_3895 {
() => {
// Module: crate::sqlite::query_builder::limit_offset
// Provides: {"impl_3895"}
// Dependencies: {}
impl < L , O > QueryFragment < Sqlite > for LimitOffsetClause < LimitClause < L > , OffsetClause < O > > where LimitClause < L > : QueryFragment < Sqlite > , OffsetClause < O > : QueryFragment < Sqlite > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Sqlite >) -> QueryResult < () > { self . limit_clause . walk_ast (out . reborrow ()) ? ; self . offset_clause . walk_ast (out . reborrow ()) ? ; Ok (()) } }
};
}
