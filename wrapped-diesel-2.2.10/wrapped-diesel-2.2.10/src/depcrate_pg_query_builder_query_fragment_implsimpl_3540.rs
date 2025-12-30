// Generated macro for impl_3540 (impl)
macro_rules! Depcrate_pg_query_builder_query_fragment_implsimpl_3540 {
() => {
// Module: crate::pg::query_builder::query_fragment_impls
// Provides: {"impl_3540"}
// Dependencies: {}
impl < T , U > QueryFragment < Pg , PgStyleArrayComparison > for NotIn < T , U > where T : QueryFragment < Pg > , U : QueryFragment < Pg > + MaybeEmpty , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Pg >) -> QueryResult < () > { self . left . walk_ast (out . reborrow ()) ? ; out . push_sql (" != ALL(") ; self . values . walk_ast (out . reborrow ()) ? ; out . push_sql (")") ; Ok (()) } }
};
}
