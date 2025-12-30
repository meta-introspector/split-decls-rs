// Generated macro for impl_3460 (impl)
macro_rules! Depcrate_pg_query_builder_copyimpl_3460 {
() => {
// Module: crate::pg::query_builder::copy
// Provides: {"impl_3460"}
// Dependencies: {}
impl < T > CopyTarget for T where T : Table + StaticQueryFragment , T :: SqlType : SqlType , T :: AllColumns : ColumnList , T :: Component : QueryFragment < Pg > , { type Table = Self ; type SqlType = T :: SqlType ; fn walk_target (mut pass : crate :: query_builder :: AstPass < '_ , '_ , Pg >) -> crate :: QueryResult < () > { T :: STATIC_COMPONENT . walk_ast (pass . reborrow ()) ? ; pass . push_sql ("(") ; T :: all_columns () . walk_ast (pass . reborrow ()) ? ; pass . push_sql (")") ; Ok (()) } }
};
}
