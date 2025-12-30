// Generated macro for impl_3911 (impl)
macro_rules! Depcrate_sqlite_query_builder_query_fragment_implsimpl_3911 {
() => {
// Module: crate::sqlite::query_builder::query_fragment_impls
// Provides: {"impl_3911"}
// Dependencies: {}
# [cfg (feature = "sqlite")] impl < F , S , D , W , O , LOf , G , H , LC > QueryFragment < crate :: sqlite :: Sqlite > for OnConflictSelectWrapper < SelectStatement < F , S , D , WhereClause < W > , O , LOf , G , H , LC > > where SelectStatement < F , S , D , WhereClause < W > , O , LOf , G , H , LC > : QueryFragment < crate :: sqlite :: Sqlite > , { fn walk_ast < 'b > (& 'b self , out : AstPass < '_ , 'b , crate :: sqlite :: Sqlite >) -> QueryResult < () > { self . 0 . walk_ast (out) } }
};
}
