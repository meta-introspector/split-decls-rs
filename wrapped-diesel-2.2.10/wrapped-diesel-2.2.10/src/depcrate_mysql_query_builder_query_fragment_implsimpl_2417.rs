// Generated macro for impl_2417 (impl)
macro_rules! Depcrate_mysql_query_builder_query_fragment_implsimpl_2417 {
() => {
// Module: crate::mysql::query_builder::query_fragment_impls
// Provides: {"impl_2417"}
// Dependencies: {}
impl < L , R > QueryFragment < Mysql , crate :: mysql :: backend :: MysqlConcatClause > for Concat < L , R > where L : QueryFragment < Mysql > , R : QueryFragment < Mysql > , { fn walk_ast < 'b > (& 'b self , mut out : crate :: query_builder :: AstPass < '_ , 'b , Mysql > ,) -> crate :: result :: QueryResult < () > { out . push_sql ("CONCAT(") ; self . left . walk_ast (out . reborrow ()) ? ; out . push_sql (",") ; self . right . walk_ast (out . reborrow ()) ? ; out . push_sql (")") ; Ok (()) } }
};
}
