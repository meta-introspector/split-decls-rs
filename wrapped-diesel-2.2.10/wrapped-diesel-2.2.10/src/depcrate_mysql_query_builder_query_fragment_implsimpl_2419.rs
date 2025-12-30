// Generated macro for impl_2419 (impl)
macro_rules! Depcrate_mysql_query_builder_query_fragment_implsimpl_2419 {
() => {
// Module: crate::mysql::query_builder::query_fragment_impls
// Provides: {"impl_2419"}
// Dependencies: {}
impl < T , Tab > QueryFragment < Mysql , crate :: mysql :: backend :: MysqlOnConflictClause > for DoUpdate < T , Tab > where T : QueryFragment < Mysql > , Tab : Table + StaticQueryFragment , Tab :: PrimaryKey : DoNothingClauseHelper , Tab :: Component : QueryFragment < Mysql > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Mysql >) -> QueryResult < () > { out . unsafe_to_cache_prepared () ; out . push_sql (" UPDATE ") ; if self . changeset . is_noop (out . backend ()) ? { Tab :: PrimaryKey :: walk_ast :: < Tab > (out . reborrow ()) ? ; } else { self . changeset . walk_ast (out . reborrow ()) ? ; } Ok (()) } }
};
}
