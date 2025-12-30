// Generated macro for impl_2420 (impl)
macro_rules! Depcrate_mysql_query_builder_query_fragment_implsimpl_2420 {
() => {
// Module: crate::mysql::query_builder::query_fragment_impls
// Provides: {"impl_2420"}
// Dependencies: {}
impl < Values , Target , Action > QueryFragment < Mysql , MysqlOnConflictClause > for OnConflictValues < Values , Target , Action , NoWhereClause > where Values : QueryFragment < Mysql > , Target : QueryFragment < Mysql > , Action : QueryFragment < Mysql > , NoWhereClause : QueryFragment < Mysql > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Mysql >) -> QueryResult < () > { self . values . walk_ast (out . reborrow ()) ? ; out . push_sql (" ON DUPLICATE KEY") ; self . target . walk_ast (out . reborrow ()) ? ; self . action . walk_ast (out . reborrow ()) ? ; self . where_clause . walk_ast (out) ? ; Ok (()) } }
};
}
