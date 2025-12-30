// Generated macro for impl_2389 (impl)
macro_rules! Depcrate_mysql_query_builder_limit_offsetimpl_2389 {
() => {
// Module: crate::mysql::query_builder::limit_offset
// Provides: {"impl_2389"}
// Dependencies: {}
impl QueryFragment < Mysql > for LimitOffsetClause < NoLimitClause , NoOffsetClause > { fn walk_ast < 'b > (& 'b self , _out : AstPass < '_ , 'b , Mysql >) -> QueryResult < () > { Ok (()) } }
};
}
