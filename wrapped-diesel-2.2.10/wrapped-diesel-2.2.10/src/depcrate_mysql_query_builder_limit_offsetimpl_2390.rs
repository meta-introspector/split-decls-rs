// Generated macro for impl_2390 (impl)
macro_rules! Depcrate_mysql_query_builder_limit_offsetimpl_2390 {
() => {
// Module: crate::mysql::query_builder::limit_offset
// Provides: {"impl_2390"}
// Dependencies: {}
impl < L > QueryFragment < Mysql > for LimitOffsetClause < LimitClause < L > , NoOffsetClause > where LimitClause < L > : QueryFragment < Mysql > , { fn walk_ast < 'b > (& 'b self , out : AstPass < '_ , 'b , Mysql >) -> QueryResult < () > { self . limit_clause . walk_ast (out) ? ; Ok (()) } }
};
}
