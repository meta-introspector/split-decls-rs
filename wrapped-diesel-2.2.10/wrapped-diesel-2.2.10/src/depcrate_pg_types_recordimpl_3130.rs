// Generated macro for impl_3130 (impl)
macro_rules! Depcrate_pg_types_recordimpl_3130 {
() => {
// Module: crate::pg::types::record
// Provides: {"impl_3130"}
// Dependencies: {}
impl < T > QueryFragment < Pg > for PgTuple < T > where T : QueryFragment < Pg > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Pg >) -> QueryResult < () > { out . push_sql ("(") ; self . 0 . walk_ast (out . reborrow ()) ? ; out . push_sql (")") ; Ok (()) } }
};
}
