// Generated macro for impl_3893 (impl)
macro_rules! Depcrate_sqlite_query_builder_limit_offsetimpl_3893 {
() => {
// Module: crate::sqlite::query_builder::limit_offset
// Provides: {"impl_3893"}
// Dependencies: {}
impl < L > QueryFragment < Sqlite > for LimitOffsetClause < LimitClause < L > , NoOffsetClause > where LimitClause < L > : QueryFragment < Sqlite > , { fn walk_ast < 'b > (& 'b self , out : AstPass < '_ , 'b , Sqlite >) -> QueryResult < () > { self . limit_clause . walk_ast (out) ? ; Ok (()) } }
};
}
