// Generated macro for impl_2391 (impl)
macro_rules! Depcrate_mysql_query_builder_limit_offsetimpl_2391 {
() => {
// Module: crate::mysql::query_builder::limit_offset
// Provides: {"impl_2391"}
// Dependencies: {}
impl < L , O > QueryFragment < Mysql > for LimitOffsetClause < LimitClause < L > , OffsetClause < O > > where LimitClause < L > : QueryFragment < Mysql > , OffsetClause < O > : QueryFragment < Mysql > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Mysql >) -> QueryResult < () > { self . limit_clause . walk_ast (out . reborrow ()) ? ; self . offset_clause . walk_ast (out . reborrow ()) ? ; Ok (()) } }
};
}
