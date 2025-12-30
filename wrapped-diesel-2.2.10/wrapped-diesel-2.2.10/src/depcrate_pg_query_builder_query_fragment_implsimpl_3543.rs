// Generated macro for impl_3543 (impl)
macro_rules! Depcrate_pg_query_builder_query_fragment_implsimpl_3543 {
() => {
// Module: crate::pg::query_builder::query_fragment_impls
// Provides: {"impl_3543"}
// Dependencies: {}
impl < S > QueryFragment < crate :: pg :: Pg > for OnConflictSelectWrapper < S > where S : QueryFragment < crate :: pg :: Pg > , { fn walk_ast < 'b > (& 'b self , out : AstPass < '_ , 'b , crate :: pg :: Pg >) -> QueryResult < () > { self . 0 . walk_ast (out) } }
};
}
