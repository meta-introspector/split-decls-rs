// Generated macro for impl_3430 (impl)
macro_rules! Depcrate_pg_query_builder_copy_copy_toimpl_3430 {
() => {
// Module: crate::pg::query_builder::copy::copy_to
// Provides: {"impl_3430"}
// Dependencies: {}
impl < S > QueryFragment < Pg > for CopyToCommand < S > where S : CopyTarget , { fn walk_ast < 'b > (& 'b self , mut pass : crate :: query_builder :: AstPass < '_ , 'b , Pg > ,) -> crate :: QueryResult < () > { pass . unsafe_to_cache_prepared () ; pass . push_sql ("COPY ") ; S :: walk_target (pass . reborrow ()) ? ; pass . push_sql (" TO STDOUT") ; self . options . walk_ast (pass . reborrow ()) ? ; Ok (()) } }
};
}
