// Generated macro for impl_2392 (impl)
macro_rules! Depcrate_mysql_query_builder_limit_offsetimpl_2392 {
() => {
// Module: crate::mysql::query_builder::limit_offset
// Provides: {"impl_2392"}
// Dependencies: {}
impl QueryFragment < Mysql > for BoxedLimitOffsetClause < '_ , Mysql > { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Mysql >) -> QueryResult < () > { match (self . limit . as_ref () , self . offset . as_ref ()) { (Some (limit) , Some (offset)) => { limit . walk_ast (out . reborrow ()) ? ; offset . walk_ast (out . reborrow ()) ? ; } (Some (limit) , None) => { limit . walk_ast (out . reborrow ()) ? ; } (None , Some (offset)) => { out . push_sql (" LIMIT 18446744073709551615 ") ; offset . walk_ast (out . reborrow ()) ? ; } (None , None) => { } } Ok (()) } }
};
}
