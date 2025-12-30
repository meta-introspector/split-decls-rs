// Generated macro for impl_3892 (impl)
macro_rules! Depcrate_sqlite_query_builder_limit_offsetimpl_3892 {
() => {
// Module: crate::sqlite::query_builder::limit_offset
// Provides: {"impl_3892"}
// Dependencies: {}
impl QueryFragment < Sqlite > for LimitOffsetClause < NoLimitClause , NoOffsetClause > { fn walk_ast < 'b > (& 'b self , _out : AstPass < '_ , 'b , Sqlite >) -> QueryResult < () > { Ok (()) } }
};
}
