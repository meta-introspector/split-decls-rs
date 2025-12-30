// Generated macro for impl_2418 (impl)
macro_rules! Depcrate_mysql_query_builder_query_fragment_implsimpl_2418 {
() => {
// Module: crate::mysql::query_builder::query_fragment_impls
// Provides: {"impl_2418"}
// Dependencies: {}
impl < T > QueryFragment < Mysql , crate :: mysql :: backend :: MysqlOnConflictClause > for DoNothing < T > where T : Table + StaticQueryFragment , T :: Component : QueryFragment < Mysql > , T :: PrimaryKey : DoNothingClauseHelper , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Mysql >) -> QueryResult < () > { out . push_sql (" UPDATE ") ; T :: PrimaryKey :: walk_ast :: < T > (out . reborrow ()) ? ; Ok (()) } }
};
}
