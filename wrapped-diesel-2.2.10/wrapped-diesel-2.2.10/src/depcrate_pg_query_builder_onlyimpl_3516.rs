// Generated macro for impl_3516 (impl)
macro_rules! Depcrate_pg_query_builder_onlyimpl_3516 {
() => {
// Module: crate::pg::query_builder::only
// Provides: {"impl_3516"}
// Dependencies: {}
impl < S > QueryFragment < Pg > for Only < S > where S : QueryFragment < Pg > , { fn walk_ast < 'b > (& 'b self , mut pass : AstPass < '_ , 'b , Pg >) -> QueryResult < () > { pass . push_sql (" ONLY ") ; self . source . walk_ast (pass . reborrow ()) ? ; Ok (()) } }
};
}
