// Generated macro for impl_3912 (impl)
macro_rules! Depcrate_sqlite_query_builder_query_fragment_implsimpl_3912 {
() => {
// Module: crate::sqlite::query_builder::query_fragment_impls
// Provides: {"impl_3912"}
// Dependencies: {}
# [cfg (feature = "sqlite")] impl < 'a , ST , QS , GB > QueryFragment < crate :: sqlite :: Sqlite > for OnConflictSelectWrapper < BoxedSelectStatement < 'a , ST , QS , crate :: sqlite :: Sqlite , GB > > where BoxedSelectStatement < 'a , ST , QS , crate :: sqlite :: Sqlite , GB > : QueryFragment < crate :: sqlite :: Sqlite > , QS : QueryFragment < crate :: sqlite :: Sqlite > , { fn walk_ast < 'b > (& 'b self , pass : AstPass < '_ , 'b , crate :: sqlite :: Sqlite >) -> QueryResult < () > { self . 0 . build_query (pass , | where_clause , mut pass | { match where_clause { BoxedWhereClause :: None => pass . push_sql (" WHERE 1=1 ") , w => w . walk_ast (pass . reborrow ()) ? , } Ok (()) }) } }
};
}
