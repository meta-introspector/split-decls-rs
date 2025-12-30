// Generated macro for impl_2412 (impl)
macro_rules! Depcrate_mysql_query_builder_query_fragment_implsimpl_2412 {
() => {
// Module: crate::mysql::query_builder::query_fragment_impls
// Provides: {"impl_2412"}
// Dependencies: {}
impl QueryFragment < Mysql > for ForShare { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Mysql >) -> QueryResult < () > { out . push_sql (" FOR SHARE") ; Ok (()) } }
};
}
