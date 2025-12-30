// Generated macro for impl_2424 (impl)
macro_rules! Depcrate_mysql_query_builder_query_fragment_implsimpl_2424 {
() => {
// Module: crate::mysql::query_builder::query_fragment_impls
// Provides: {"impl_2424"}
// Dependencies: {}
impl < S > QueryFragment < crate :: mysql :: Mysql > for OnConflictSelectWrapper < S > where S : QueryFragment < crate :: mysql :: Mysql > , { fn walk_ast < 'b > (& 'b self , out : AstPass < '_ , 'b , crate :: mysql :: Mysql >) -> QueryResult < () > { self . 0 . walk_ast (out) } }
};
}
