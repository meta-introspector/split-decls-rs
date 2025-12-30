// Generated macro for impl_2411 (impl)
macro_rules! Depcrate_mysql_query_builder_query_fragment_implsimpl_2411 {
() => {
// Module: crate::mysql::query_builder::query_fragment_impls
// Provides: {"impl_2411"}
// Dependencies: {}
impl QueryFragment < Mysql > for ForUpdate { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Mysql >) -> QueryResult < () > { out . push_sql (" FOR UPDATE") ; Ok (()) } }
};
}
