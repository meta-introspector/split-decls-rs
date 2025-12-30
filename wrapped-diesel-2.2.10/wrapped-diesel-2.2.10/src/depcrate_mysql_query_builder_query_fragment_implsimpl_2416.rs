// Generated macro for impl_2416 (impl)
macro_rules! Depcrate_mysql_query_builder_query_fragment_implsimpl_2416 {
() => {
// Module: crate::mysql::query_builder::query_fragment_impls
// Provides: {"impl_2416"}
// Dependencies: {}
impl QueryFragment < Mysql , crate :: mysql :: backend :: MysqlStyleDefaultValueClause > for DefaultValues { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Mysql >) -> QueryResult < () > { out . push_sql ("() VALUES ()") ; Ok (()) } }
};
}
