// Generated macro for impl_3894 (impl)
macro_rules! Depcrate_sqlite_query_builder_limit_offsetimpl_3894 {
() => {
// Module: crate::sqlite::query_builder::limit_offset
// Provides: {"impl_3894"}
// Dependencies: {}
impl < O > QueryFragment < Sqlite > for LimitOffsetClause < NoLimitClause , OffsetClause < O > > where OffsetClause < O > : QueryFragment < Sqlite > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Sqlite >) -> QueryResult < () > { out . push_sql (" LIMIT -1 ") ; self . offset_clause . walk_ast (out) ? ; Ok (()) } }
};
}
