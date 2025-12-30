// Generated macro for impl_3389 (impl)
macro_rules! Depcrate_pg_query_builder_copy_copy_fromimpl_3389 {
() => {
// Module: crate::pg::query_builder::copy::copy_from
// Provides: {"impl_3389"}
// Dependencies: {}
impl < S , T > QueryFragment < Pg > for InternalCopyFromQuery < S , T > where S : CopyFromExpression < T > , { fn walk_ast < 'b > (& 'b self , mut pass : crate :: query_builder :: AstPass < '_ , 'b , Pg > ,) -> crate :: QueryResult < () > { pass . unsafe_to_cache_prepared () ; pass . push_sql ("COPY ") ; self . target . walk_target (pass . reborrow ()) ? ; pass . push_sql (" FROM STDIN") ; self . target . options () . walk_ast (pass . reborrow ()) ? ; Ok (()) } }
};
}
